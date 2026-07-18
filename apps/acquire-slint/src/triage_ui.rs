//! M11 UI-facing triage entry points.

use std::path::{Path, PathBuf};

use trareon_core::{
    TriageBundle, collect_live_triage, collect_triage_from_fixture, write_triage_bundle,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriageRun {
    pub path: PathBuf,
    pub footprint_note: String,
}

pub fn run_fixture_triage(dir: &Path, out: &Path) -> Result<TriageRun, String> {
    write_bundle(
        collect_triage_from_fixture(dir).map_err(|error| error.to_string())?,
        out,
    )
}

pub fn run_live_triage(out: &Path) -> Result<TriageRun, String> {
    write_bundle(
        collect_live_triage().map_err(|error| error.to_string())?,
        out,
    )
}

fn write_bundle(bundle: TriageBundle, out: &Path) -> Result<TriageRun, String> {
    let path = write_triage_bundle(&bundle, out).map_err(|error| error.to_string())?;
    Ok(TriageRun {
        path,
        footprint_note: bundle.footprint_note,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_triage_returns_bundle_path_and_note() {
        let dir = tempfile::tempdir().unwrap();
        for (name, body) in [
            ("processes.txt", "1 init\n"),
            ("network.txt", "tcp 22\n"),
            ("users.txt", "examiner\n"),
        ] {
            std::fs::write(dir.path().join(name), body).unwrap();
        }
        let run = run_fixture_triage(dir.path(), &dir.path().join("triage.json")).unwrap();
        assert!(run.path.is_file());
        assert_eq!(run.footprint_note, "synthetic fixture");
    }
}
