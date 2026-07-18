use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

/// Predefined acquisition depth choices for the desktop workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileKind {
    Quick,
    Standard,
    Deep,
    Custom,
}

/// Outcome of the fixture-oriented quick collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileCollection {
    pub kind: ProfileKind,
    pub copied: Vec<String>,
    pub excluded: Vec<String>,
}

/// Copies explicitly selected relative files from a fixture tree and records all exclusions.
///
/// This is a logical fixture collector, not a live filesystem acquisition.
pub fn collect_quick_profile(
    fixture_root: &Path,
    output_dir: &Path,
    relative_paths: &[&str],
) -> Result<ProfileCollection, String> {
    if !fixture_root.is_dir() {
        return Err(format!(
            "fixture root does not exist: {}",
            fixture_root.display()
        ));
    }
    let requested: BTreeSet<PathBuf> = relative_paths.iter().map(PathBuf::from).collect();
    if requested
        .iter()
        .any(|path| path.is_absolute() || path.components().count() == 0)
    {
        return Err("profile paths must be non-empty and relative".to_string());
    }
    fs::create_dir_all(output_dir).map_err(|error| error.to_string())?;

    let available = list_relative_files(fixture_root)?;
    let mut copied = Vec::new();
    for path in &requested {
        let source = fixture_root.join(path);
        if !source.is_file() {
            return Err(format!(
                "requested fixture file is unavailable: {}",
                path.display()
            ));
        }
        let destination = output_dir.join(path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::copy(source, destination).map_err(|error| error.to_string())?;
        copied.push(path.to_string_lossy().replace('\\', "/"));
    }

    let excluded: Vec<String> = available
        .into_iter()
        .filter(|path| !requested.contains(&PathBuf::from(path)))
        .collect();
    let exclusions_json =
        serde_json::to_string_pretty(&excluded).map_err(|error| error.to_string())?;
    fs::write(output_dir.join("exclusions.json"), exclusions_json)
        .map_err(|error| error.to_string())?;

    Ok(ProfileCollection {
        kind: ProfileKind::Quick,
        copied,
        excluded,
    })
}

fn list_relative_files(root: &Path) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    collect_relative_files(root, root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_relative_files(
    root: &Path,
    current: &Path,
    files: &mut Vec<String>,
) -> Result<(), String> {
    for entry in fs::read_dir(current).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_relative_files(root, &path, files)?;
        } else if path.is_file() {
            files.push(
                path.strip_prefix(root)
                    .map_err(|error| error.to_string())?
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_profile_copies_requested_files_and_records_exclusions() {
        let dir = tempfile::tempdir().unwrap();
        let fixture = dir.path().join("fixture");
        std::fs::create_dir_all(fixture.join("nested")).unwrap();
        std::fs::write(fixture.join("wanted.txt"), "wanted").unwrap();
        std::fs::write(fixture.join("nested/also-wanted.txt"), "also").unwrap();
        std::fs::write(fixture.join("not-requested.txt"), "skip").unwrap();
        let output = dir.path().join("output");

        let result =
            collect_quick_profile(&fixture, &output, &["wanted.txt", "nested/also-wanted.txt"])
                .unwrap();

        assert_eq!(result.copied, vec!["nested/also-wanted.txt", "wanted.txt"]);
        assert_eq!(
            std::fs::read_to_string(output.join("wanted.txt")).unwrap(),
            "wanted"
        );
        let exclusions = std::fs::read_to_string(output.join("exclusions.json")).unwrap();
        assert!(exclusions.contains("not-requested.txt"));
    }
}
