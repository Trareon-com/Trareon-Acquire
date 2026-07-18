//! M12 two-source acquisition orchestration with a conservative resource governor.

use std::{
    fs,
    path::{Path, PathBuf},
    thread,
};

use serde::Serialize;

pub const PARALLEL_SIZE_THRESHOLD: u64 = 4 * 1024 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum GovernorRecommendation {
    Sequential,
    Parallel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceGovernor {
    pub total_bytes: u64,
    pub threshold_bytes: u64,
    pub recommendation: GovernorRecommendation,
}

impl ResourceGovernor {
    pub fn recommend(a: &Path, b: &Path) -> Result<Self, String> {
        let total_bytes = fs::metadata(a).map_err(|error| error.to_string())?.len()
            + fs::metadata(b).map_err(|error| error.to_string())?.len();
        Ok(Self {
            total_bytes,
            threshold_bytes: PARALLEL_SIZE_THRESHOLD,
            recommendation: if total_bytes > PARALLEL_SIZE_THRESHOLD {
                GovernorRecommendation::Sequential
            } else {
                GovernorRecommendation::Parallel
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSourceRun {
    pub first_package: PathBuf,
    pub second_package: PathBuf,
    pub governor_note: PathBuf,
    pub recommendation: GovernorRecommendation,
}

pub fn run_parallel_two_sources(a: &Path, b: &Path, out: &Path) -> Result<MultiSourceRun, String> {
    let governor = ResourceGovernor::recommend(a, b)?;
    fs::create_dir_all(out).map_err(|error| error.to_string())?;
    let first = out.join("source-a");
    let second = out.join("source-b");

    let (first_package, second_package) = match governor.recommendation {
        GovernorRecommendation::Sequential => (
            crate::run_foundation_demo(a, &first, None)
                .map(|result| PathBuf::from(result.package_path))?,
            crate::run_foundation_demo(b, &second, None)
                .map(|result| PathBuf::from(result.package_path))?,
        ),
        GovernorRecommendation::Parallel => {
            let a = a.to_path_buf();
            let first_for_thread = first.clone();
            let worker = thread::spawn(move || {
                crate::run_foundation_demo(&a, &first_for_thread, None)
                    .map(|result| PathBuf::from(result.package_path))
            });
            let second_package = crate::run_foundation_demo(b, &second, None)
                .map(|result| PathBuf::from(result.package_path))?;
            let first_package = worker
                .join()
                .map_err(|_| "first acquisition worker panicked".to_string())??;
            (first_package, second_package)
        }
    };
    let governor_note = out.join("resource-governor.json");
    fs::write(
        &governor_note,
        serde_json::to_vec_pretty(&governor).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    Ok(MultiSourceRun {
        first_package,
        second_package,
        governor_note,
        recommendation: governor.recommendation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sources_run_and_record_governor_decision() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.bin");
        let b = dir.path().join("b.bin");
        fs::write(&a, b"one").unwrap();
        fs::write(&b, b"two").unwrap();
        let run = run_parallel_two_sources(&a, &b, &dir.path().join("out")).unwrap();
        assert_eq!(run.recommendation, GovernorRecommendation::Parallel);
        assert!(run.first_package.is_dir());
        assert!(run.second_package.is_dir());
        assert!(run.governor_note.is_file());
    }
}
