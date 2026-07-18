//! M13 quality-management self checks and portable validation artifacts.

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MethodBadge {
    Validated,
    Limited,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationCheck {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationMatrix {
    pub schema: String,
    pub build_identity: String,
    pub badge: MethodBadge,
    pub checks: Vec<ValidationCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedMatrixExports {
    pub matrix_json: PathBuf,
    pub signed_json: PathBuf,
    pub html: PathBuf,
}

pub fn self_test() -> ValidationCheck {
    let result = (|| -> Result<(), String> {
        let dir = tempfile::tempdir().map_err(|error| error.to_string())?;
        let package = crate::run_synthetic_demo(dir.path())?;
        crate::tools_hub::verify_package(package.as_path()).map(|_| ())
    })();
    match result {
        Ok(()) => ValidationCheck {
            name: "synthetic acquisition and verification".into(),
            passed: true,
            detail: "synthetic package verified".into(),
        },
        Err(error) => ValidationCheck {
            name: "synthetic acquisition and verification".into(),
            passed: false,
            detail: error,
        },
    }
}

pub fn known_dataset_suite() -> ValidationCheck {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/fsnap-v0.1/valid");
    if !fixture.is_dir() {
        return ValidationCheck {
            name: "known dataset verification".into(),
            passed: false,
            detail: "fixture unavailable".into(),
        };
    }
    match crate::tools_hub::verify_package(&fixture) {
        Ok(_) => ValidationCheck {
            name: "known dataset verification".into(),
            passed: true,
            detail: fixture.display().to_string(),
        },
        Err(error) => ValidationCheck {
            name: "known dataset verification".into(),
            passed: false,
            detail: error,
        },
    }
}

pub fn validation_matrix() -> ValidationMatrix {
    let checks = vec![self_test(), known_dataset_suite()];
    let badge = if !checks[0].passed {
        MethodBadge::Limited
    } else if checks.iter().all(|check| check.passed) {
        MethodBadge::Validated
    } else {
        MethodBadge::Limited
    };
    ValidationMatrix {
        schema: "trareon.qms.validation-matrix/1".into(),
        build_identity: trareon_core::build_identity().into(),
        badge,
        checks,
    }
}

pub fn export_validation_pack(dir: &Path) -> Result<SignedMatrixExports, String> {
    export_signed_matrix_json(dir)
}

pub fn export_signed_matrix_json(dir: &Path) -> Result<SignedMatrixExports, String> {
    fs::create_dir_all(dir).map_err(|error| error.to_string())?;
    let matrix = validation_matrix();
    let matrix_bytes = serde_json::to_vec_pretty(&matrix).map_err(|error| error.to_string())?;
    let matrix_json = dir.join("validation-matrix.json");
    fs::write(&matrix_json, &matrix_bytes).map_err(|error| error.to_string())?;
    let seal = MatrixSeal {
        schema: "trareon.qms.matrix-seal/1",
        build_identity: trareon_core::build_identity(),
        matrix_sha256: sha256(&matrix_bytes),
    };
    let signed_json = dir.join("validation-matrix.seal.json");
    fs::write(
        &signed_json,
        serde_json::to_vec_pretty(&seal).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let html = export_signed_matrix_html(dir, &matrix, &seal)?;
    Ok(SignedMatrixExports {
        matrix_json,
        signed_json,
        html,
    })
}

pub fn export_signed_matrix_html(
    dir: &Path,
    matrix: &ValidationMatrix,
    seal: &MatrixSeal,
) -> Result<PathBuf, String> {
    let rows = matrix
        .checks
        .iter()
        .map(|check| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                check.name, check.passed, check.detail
            )
        })
        .collect::<String>();
    let path = dir.join("validation-matrix.html");
    fs::write(
        &path,
        format!(
            "<!doctype html><meta charset=\"utf-8\"><title>Validation Matrix</title>\
             <h1>Validation Matrix</h1><p>Build: {}</p><p>Badge: {:?}</p>\
             <p>SHA-256: {}</p><table><tr><th>Check</th><th>Passed</th><th>Detail</th></tr>{rows}</table>",
            seal.build_identity, matrix.badge, seal.matrix_sha256
        ),
    )
    .map_err(|error| error.to_string())?;
    Ok(path)
}

#[derive(Debug, Serialize)]
pub struct MatrixSeal {
    pub schema: &'static str,
    pub build_identity: &'static str,
    pub matrix_sha256: String,
}

fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_pack_contains_hashed_matrix() {
        let dir = tempfile::tempdir().unwrap();
        let exports = export_validation_pack(dir.path()).unwrap();
        assert!(exports.matrix_json.is_file());
        assert!(exports.signed_json.is_file());
        assert!(exports.html.is_file());
        assert!(
            std::fs::read_to_string(exports.signed_json)
                .unwrap()
                .contains("matrix_sha256")
        );
    }
}
