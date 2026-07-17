//! Privileged acquisition broker protocol spike (Track C).
//!
//! Typed request/response surface only. No OS elevation, no raw-disk I/O,
//! no shell. Unknown operations and shell-like payloads are rejected.
//! Optional lab allowlist binding denies system disks and non-listed block
//! identities before the stub returns `NotImplemented`.

use sha2::{Digest, Sha256};

use crate::{
    CoreError,
    lab_policy::{self, LabAllowlist},
};

/// Allowlisted broker operations. Anything else is rejected.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrokerOperation {
    /// Open a previously allowlisted source identity for read-only streaming.
    OpenAllowlistedSource,
    /// Read the next bounded range from an open source handle.
    ReadRange,
    /// Close an open source handle.
    CloseSource,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BrokerRequest {
    pub operation_id: String,
    pub operation: BrokerOperation,
    pub plan_digest: String,
    pub source_identity: String,
    pub session_token: String,
    pub nonce: String,
    pub request_id: String,
    /// Inclusive start offset for `ReadRange`; ignored otherwise.
    pub range_start: Option<u64>,
    /// Exclusive end offset for `ReadRange`; ignored otherwise.
    pub range_end: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum BrokerResponse {
    Accepted { request_id: String },
    Denied { request_id: String, reason: String },
    NotImplemented { request_id: String, reason: String },
}

fn looks_like_shell(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("cmd.exe")
        || lower.contains("/bin/sh")
        || lower.contains("/bin/bash")
        || lower.contains("powershell")
        || lower.contains(";")
        || lower.contains("|")
        || lower.contains("`")
        || lower.contains("$(")
}

fn is_hex_digest(value: &str, expected_len: usize) -> bool {
    value.len() == expected_len && value.chars().all(|c| c.is_ascii_hexdigit())
}

/// Validate and classify a broker request without performing privilege.
///
/// Without an allowlist, block-device-like `source_identity` values are denied.
/// Structurally valid allowlisted ops still return `NotImplemented` until a
/// reviewed helper exists.
pub fn evaluate_broker_request(request: &BrokerRequest) -> Result<BrokerResponse, CoreError> {
    evaluate_broker_request_with_allowlist(request, None)
}

/// Same as [`evaluate_broker_request`], with optional human-approved allowlist.
pub fn evaluate_broker_request_with_allowlist(
    request: &BrokerRequest,
    allowlist: Option<&LabAllowlist>,
) -> Result<BrokerResponse, CoreError> {
    if request.operation_id.trim().is_empty()
        || request.session_token.trim().is_empty()
        || request.nonce.trim().is_empty()
        || request.request_id.trim().is_empty()
    {
        return Ok(BrokerResponse::Denied {
            request_id: request.request_id.clone(),
            reason: "missing required authentication fields".to_string(),
        });
    }
    if !is_hex_digest(&request.plan_digest, 64) {
        return Ok(BrokerResponse::Denied {
            request_id: request.request_id.clone(),
            reason: "plan_digest must be a 64-char hex SHA-256".to_string(),
        });
    }
    if request.source_identity.trim().is_empty() || looks_like_shell(&request.source_identity) {
        return Ok(BrokerResponse::Denied {
            request_id: request.request_id.clone(),
            reason: "source_identity rejected (empty or shell-like)".to_string(),
        });
    }
    if looks_like_shell(&request.operation_id)
        || looks_like_shell(&request.session_token)
        || looks_like_shell(&request.request_id)
    {
        return Ok(BrokerResponse::Denied {
            request_id: request.request_id.clone(),
            reason: "shell-like payload rejected".to_string(),
        });
    }
    if let (Some(start), Some(end)) = (request.range_start, request.range_end)
        && start >= end
    {
        return Ok(BrokerResponse::Denied {
            request_id: request.request_id.clone(),
            reason: "invalid read range".to_string(),
        });
    }

    if let Err(error) =
        lab_policy::assert_broker_source_identity(&request.source_identity, allowlist)
    {
        return Ok(BrokerResponse::Denied {
            request_id: request.request_id.clone(),
            reason: error.to_string(),
        });
    }

    match request.operation {
        BrokerOperation::OpenAllowlistedSource
        | BrokerOperation::ReadRange
        | BrokerOperation::CloseSource => Ok(BrokerResponse::NotImplemented {
            request_id: request.request_id.clone(),
            reason: "privileged helper not implemented; protocol accepted structurally".to_string(),
        }),
    }
}

/// Deterministic plan digest helper for tests and future signed plans.
pub fn plan_digest(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

/// Elevation helper contract for a future privileged broker process.
///
/// Implementations must not shell out and must not claim OS elevation until a
/// reviewed lab helper exists. The stub always returns `NotImplemented`.
pub trait ElevationHelper: Send + Sync {
    fn elevate_for(&self, request: &BrokerRequest) -> Result<BrokerResponse, CoreError>;
}

/// Spike-only helper: protocol-aware, never elevates.
///
/// Optionally carries a lab allowlist so block-device identities are denied
/// before the stub returns `NotImplemented`.
#[derive(Debug, Default, Clone)]
pub struct StubElevationHelper {
    allowlist: Option<LabAllowlist>,
}

impl StubElevationHelper {
    pub fn with_allowlist(allowlist: LabAllowlist) -> Self {
        Self {
            allowlist: Some(allowlist),
        }
    }
}

impl ElevationHelper for StubElevationHelper {
    fn elevate_for(&self, request: &BrokerRequest) -> Result<BrokerResponse, CoreError> {
        match evaluate_broker_request_with_allowlist(request, self.allowlist.as_ref())? {
            BrokerResponse::Accepted { request_id } => Ok(BrokerResponse::NotImplemented {
                request_id,
                reason: "StubElevationHelper does not perform OS elevation".to_string(),
            }),
            other => Ok(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_policy::{LAB_ALLOWLIST_SCHEMA, LabAllowlist, LabAllowlistEntry};

    fn valid_request() -> BrokerRequest {
        BrokerRequest {
            operation_id: "op-1".to_string(),
            operation: BrokerOperation::OpenAllowlistedSource,
            plan_digest: plan_digest(b"lab-plan-v1"),
            source_identity: "lab:allowlisted:disk-serial-redacted".to_string(),
            session_token: "session-token-1".to_string(),
            nonce: "nonce-1".to_string(),
            request_id: "req-1".to_string(),
            range_start: None,
            range_end: None,
        }
    }

    fn tiny11_allowlist() -> LabAllowlist {
        LabAllowlist {
            schema: LAB_ALLOWLIST_SCHEMA.to_string(),
            human_approved: true,
            approved_by: "operator".to_string(),
            entries: vec![LabAllowlistEntry {
                source_identity: "/dev/rdisk10".to_string(),
                notes: "lab".to_string(),
            }],
        }
    }

    #[test]
    fn allowlisted_op_is_structurally_accepted_as_not_implemented() {
        let response = evaluate_broker_request(&valid_request()).unwrap();
        assert!(matches!(
            response,
            BrokerResponse::NotImplemented { request_id, .. } if request_id == "req-1"
        ));
    }

    #[test]
    fn shell_like_source_is_denied() {
        let mut request = valid_request();
        request.source_identity = "/bin/sh -c evil".to_string();
        let response = evaluate_broker_request(&request).unwrap();
        assert!(matches!(response, BrokerResponse::Denied { .. }));
    }

    #[test]
    fn missing_plan_digest_is_denied() {
        let mut request = valid_request();
        request.plan_digest = "not-a-digest".to_string();
        let response = evaluate_broker_request(&request).unwrap();
        assert!(matches!(response, BrokerResponse::Denied { .. }));
    }

    #[test]
    fn invalid_range_is_denied() {
        let mut request = valid_request();
        request.operation = BrokerOperation::ReadRange;
        request.range_start = Some(10);
        request.range_end = Some(10);
        let response = evaluate_broker_request(&request).unwrap();
        assert!(matches!(response, BrokerResponse::Denied { .. }));
    }

    #[test]
    fn stub_elevation_helper_never_claims_privilege() {
        let helper = StubElevationHelper::default();
        let response = helper.elevate_for(&valid_request()).unwrap();
        assert!(matches!(response, BrokerResponse::NotImplemented { .. }));
    }

    #[test]
    fn system_disk_broker_source_is_denied() {
        let mut request = valid_request();
        request.source_identity = "/dev/rdisk0".to_string();
        let response = evaluate_broker_request(&request).unwrap();
        assert!(
            matches!(response, BrokerResponse::Denied { reason, .. } if reason.contains("hard-denied"))
        );
    }

    #[test]
    fn windows_physical_drive_zero_broker_source_is_denied() {
        let mut request = valid_request();
        request.source_identity = r"\\.\PhysicalDrive0".to_string();
        let response = evaluate_broker_request(&request).unwrap();
        assert!(
            matches!(response, BrokerResponse::Denied { reason, .. } if reason.contains("hard-denied"))
        );
    }

    #[test]
    fn block_device_without_allowlist_is_denied() {
        let mut request = valid_request();
        request.source_identity = "/dev/rdisk10".to_string();
        let response = evaluate_broker_request(&request).unwrap();
        assert!(matches!(response, BrokerResponse::Denied { .. }));
    }

    #[test]
    fn allowlisted_block_device_is_not_implemented_not_denied() {
        let mut request = valid_request();
        request.source_identity = "/dev/rdisk10".to_string();
        let response =
            evaluate_broker_request_with_allowlist(&request, Some(&tiny11_allowlist())).unwrap();
        assert!(matches!(response, BrokerResponse::NotImplemented { .. }));
    }

    #[test]
    fn stub_with_allowlist_still_does_not_elevate() {
        let helper = StubElevationHelper::with_allowlist(tiny11_allowlist());
        let mut request = valid_request();
        request.source_identity = "/dev/rdisk10".to_string();
        let response = helper.elevate_for(&request).unwrap();
        assert!(matches!(response, BrokerResponse::NotImplemented { .. }));
    }
}
