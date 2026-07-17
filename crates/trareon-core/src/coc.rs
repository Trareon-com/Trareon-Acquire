//! Chain of custody helpers: Evidence ID, QR, optional RFC3161 placeholder.

use std::path::Path;

use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::CoreError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceId {
    pub case_id: String,
    pub media_num: u16,
    pub seq_num: u16,
}

impl EvidenceId {
    pub fn new(case_id: impl Into<String>, media_num: u16, seq_num: u16) -> Self {
        Self {
            case_id: case_id.into(),
            media_num,
            seq_num,
        }
    }

    /// Default pattern: `TR-{case}-M{media:02}-S{seq:03}`
    pub fn format_id(&self) -> String {
        format!(
            "TR-{}-M{:02}-S{:03}",
            self.case_id, self.media_num, self.seq_num
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceCoC {
    pub evidence_id: String,
    pub sha256: String,
    pub acquired_at: String,
    pub examiner: String,
    pub case_ref: String,
    pub device_identifier: String,
    /// Hex SHA-256 of (evidence_id || sha256) — envelope seal without ed25519 dependency.
    pub seal_hex: String,
    pub rfc3161_status: String,
}

impl EvidenceCoC {
    pub fn build(
        id: &EvidenceId,
        sha256: &str,
        acquired_at: &str,
        examiner: &str,
        device_identifier: &str,
    ) -> Self {
        let evidence_id = id.format_id();
        let mut hasher = Sha256::new();
        hasher.update(evidence_id.as_bytes());
        hasher.update(b"|");
        hasher.update(sha256.as_bytes());
        let seal_hex = hex::encode(hasher.finalize());
        Self {
            evidence_id,
            sha256: sha256.to_string(),
            acquired_at: acquired_at.to_string(),
            examiner: examiner.to_string(),
            case_ref: id.case_id.clone(),
            device_identifier: device_identifier.to_string(),
            seal_hex,
            rfc3161_status: "skipped_offline".into(),
        }
    }

    pub fn qr_payload(&self) -> String {
        format!(
            "trareon-coc/1\nid={}\nsha256={}\nseal={}\nat={}",
            self.evidence_id, self.sha256, self.seal_hex, self.acquired_at
        )
    }

    pub fn to_qr_png(&self) -> Result<Vec<u8>, CoreError> {
        let code = QrCode::new(self.qr_payload().as_bytes())
            .map_err(|e| CoreError::Verification(format!("QR encode: {e}")))?;
        let image = code.render::<image::Luma<u8>>().max_dimensions(300, 300).build();
        let mut png = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png);
        image::DynamicImage::ImageLuma8(image)
            .write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| CoreError::Io(e.to_string()))?;
        Ok(png)
    }

    pub fn write_qr_png(&self, path: &Path) -> Result<(), CoreError> {
        let png = self.to_qr_png()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| CoreError::Io(e.to_string()))?;
        }
        std::fs::write(path, png).map_err(|e| CoreError::Io(e.to_string()))
    }

    /// Optional RFC3161: offline by default. Set `TRAREON_TSA_URL` to attempt later.
    pub fn with_rfc3161_attempt(mut self) -> Self {
        match std::env::var("TRAREON_TSA_URL") {
            Ok(url) if !url.trim().is_empty() => {
                self.rfc3161_status = format!("configured_but_not_submitted:{url}");
            }
            _ => self.rfc3161_status = "skipped_offline".into(),
        }
        self
    }

    /// Print-ready HTML sticker (small label).
    pub fn sticker_html(&self) -> String {
        format!(
            r#"<!doctype html><html><body style="font-family:monospace;padding:12px">
<h2>Trareon Evidence</h2>
<div><b>ID</b> {}</div>
<div><b>SHA-256</b> {}</div>
<div><b>Examiner</b> {}</div>
<div><b>Acquired</b> {}</div>
<div><b>Seal</b> {}</div>
</body></html>"#,
            self.evidence_id, self.sha256, self.examiner, self.acquired_at, self.seal_hex
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_id_format() {
        let id = EvidenceId::new("2026-001", 1, 1);
        assert_eq!(id.format_id(), "TR-2026-001-M01-S001");
    }

    #[test]
    fn qr_png_non_empty() {
        let id = EvidenceId::new("CASE", 2, 3);
        let coc = EvidenceCoC::build(&id, &"a".repeat(64), "2026-07-18T00:00:00Z", "lab", "serial");
        let png = coc.to_qr_png().unwrap();
        assert!(png.starts_with(b"\x89PNG"));
        assert!(png.len() > 100);
    }
}
