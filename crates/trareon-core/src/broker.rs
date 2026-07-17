//! Privileged acquisition broker protocol spike (Track C).
//!
//! Typed request/response surface only. No OS elevation, no raw-disk I/O,
//! no shell. Unknown operations and shell-like payloads are rejected.

use sha2::{Digest, Sha256};

use crate::CoreError;

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
/// Returns `Accepted` only for structurally valid allowlisted ops. Execution
/// of real elevation remains `NotImplemented` until a reviewed helper exists.
pub fn evaluate_broker_request(request: &BrokerRequest) -> Result<BrokerResponse, CoreError> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
