use std::{fs, io::Write, path::Path};

use trareon_core::{EvidenceSegmentV1, FsnapManifestV1, verify_fsnap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageInfo {
    pub schema: String,
    pub build_identity: String,
    pub evidence_sha256: String,
    pub evidence_size: u64,
    pub evidence_path: String,
    pub segments: usize,
    pub audit_root: String,
    pub audit_events: usize,
}

pub fn verify(package: &Path) -> Result<FsnapManifestV1, String> {
    verify_fsnap(package).map_err(|error| error.to_string())
}

pub fn hash_only(package: &Path) -> Result<(String, u64), String> {
    let manifest = load_manifest(package)?;
    Ok((manifest.evidence_sha256, manifest.evidence_size))
}

pub fn info(package: &Path) -> Result<PackageInfo, String> {
    let manifest = load_manifest(package)?;
    let audit_events = fs::read_to_string(package.join(&manifest.audit_relative_path))
        .map(|text| text.lines().count())
        .unwrap_or(0);
    Ok(PackageInfo {
        schema: manifest.schema,
        build_identity: manifest.build_identity,
        evidence_sha256: manifest.evidence_sha256,
        evidence_size: manifest.evidence_size,
        evidence_path: manifest.evidence_relative_path,
        segments: manifest.evidence_segments.as_ref().map_or(1, Vec::len),
        audit_root: manifest.audit_root,
        audit_events,
    })
}

pub fn compare(a: &Path, b: &Path) -> Result<bool, String> {
    let a = verify(a)?;
    let b = verify(b)?;
    Ok(a.evidence_sha256 == b.evidence_sha256
        && a.evidence_size == b.evidence_size
        && a.audit_root == b.audit_root)
}

pub fn export_segments(
    package: &Path,
    segments: &[EvidenceSegmentV1],
    output: &Path,
) -> Result<(), String> {
    let mut file = fs::File::create(output).map_err(|error| error.to_string())?;
    for segment in segments {
        let bytes =
            fs::read(package.join(&segment.relative_path)).map_err(|error| error.to_string())?;
        file.write_all(&bytes).map_err(|error| error.to_string())?;
    }
    file.sync_all().map_err(|error| error.to_string())
}

pub fn export(package: &Path, output: &Path) -> Result<(), String> {
    let manifest = verify(package)?;
    if let Some(segments) = &manifest.evidence_segments {
        export_segments(package, segments, output)
    } else {
        fs::copy(package.join(manifest.evidence_relative_path), output)
            .map(|_| ())
            .map_err(|error| error.to_string())
    }
}

fn load_manifest(package: &Path) -> Result<FsnapManifestV1, String> {
    let bytes =
        fs::read(package.join("manifest/manifest.json")).map_err(|error| error.to_string())?;
    serde_json::from_slice(&bytes).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_only_reads_manifest_without_verifying_payload() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("manifest")).unwrap();
        fs::write(
            dir.path().join("manifest/manifest.json"),
            r#"{"schema":"trareon.fsnap.manifest/1","build_identity":"trareon-acquire-foundation-v1","evidence_relative_path":"missing","evidence_size":9,"evidence_sha256":"abc","audit_relative_path":"audit","audit_sha256":"x","audit_root":"x"}"#,
        )
        .unwrap();
        assert_eq!(hash_only(dir.path()).unwrap(), ("abc".to_string(), 9));
    }
}
