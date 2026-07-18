use serde::{Deserialize, Serialize};

use crate::FsnapManifestV1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageState {
    ValidatedPartial,
    NotReported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageSummary {
    pub imaged_bytes: u64,
    pub total_bytes: u64,
    pub gaps: Vec<String>,
    pub errors: Vec<String>,
    pub state: CoverageState,
}

impl CoverageSummary {
    pub fn from_manifest(manifest: &FsnapManifestV1) -> Self {
        match &manifest.coverage {
            Some(coverage) => Self {
                imaged_bytes: coverage.imaged_bytes,
                total_bytes: coverage.total_bytes,
                gaps: coverage.gaps.clone(),
                errors: coverage.errors.clone(),
                state: CoverageState::ValidatedPartial,
            },
            None => Self {
                imaged_bytes: 0,
                total_bytes: 0,
                gaps: Vec::new(),
                errors: Vec::new(),
                state: CoverageState::NotReported,
            },
        }
    }
}

pub fn coverage_from_manifest(manifest: &FsnapManifestV1) -> CoverageSummary {
    CoverageSummary::from_manifest(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FsnapCoverageV1;

    #[test]
    fn no_coverage_is_not_reported() {
        let manifest: FsnapManifestV1 = serde_json::from_str(
            r#"{"schema":"trareon.fsnap.manifest/1","build_identity":"trareon-acquire-foundation-v1","evidence_relative_path":"e","evidence_size":0,"evidence_sha256":"h","audit_relative_path":"a","audit_sha256":"h","audit_root":"h"}"#,
        )
        .unwrap();
        assert_eq!(
            coverage_from_manifest(&manifest).state,
            CoverageState::NotReported
        );
    }

    #[test]
    fn reported_coverage_is_copied() {
        let manifest: FsnapManifestV1 = serde_json::from_str(
            r#"{"schema":"trareon.fsnap.manifest/1","build_identity":"trareon-acquire-foundation-v1","evidence_relative_path":"e","evidence_size":0,"evidence_sha256":"h","audit_relative_path":"a","audit_sha256":"h","audit_root":"h","coverage":{"imaged_bytes":8,"total_bytes":10,"gaps":["tail"],"errors":[]}}"#,
        )
        .unwrap();
        assert_eq!(
            manifest.coverage,
            Some(FsnapCoverageV1 {
                imaged_bytes: 8,
                total_bytes: 10,
                gaps: vec!["tail".into()],
                errors: vec![]
            })
        );
        assert_eq!(
            coverage_from_manifest(&manifest).state,
            CoverageState::ValidatedPartial
        );
    }
}
