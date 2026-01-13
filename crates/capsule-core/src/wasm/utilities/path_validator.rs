use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FileAccessMode {
    ReadOnly,

    #[default]
    ReadWrite,
}

#[derive(Debug)]
pub struct ParsedPath {
    pub path: PathBuf,
    pub guest_path: String,
    pub mode: FileAccessMode,
}

#[derive(Debug)]
pub enum PathValidationError {
    AbsolutePathNotAllowed(String),
    EscapesProjectDirectory(String),
    PathNotFound(String),
    InvalidMode(String),
}

impl fmt::Display for PathValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathValidationError::AbsolutePathNotAllowed(path) => {
                write!(f, "Absolute paths are not allowed: {}", path)
            }
            PathValidationError::EscapesProjectDirectory(path) => {
                write!(f, "Path escapes project directory: {}", path)
            }
            PathValidationError::PathNotFound(path) => {
                write!(f, "Path does not exist: {}", path)
            }
            PathValidationError::InvalidMode(mode) => {
                write!(
                    f,
                    "Invalid access mode '{}'. Use :ro (read-only) or :rw (read-write)",
                    mode
                )
            }
        }
    }
}

impl Error for PathValidationError {}

fn parse_path_with_mode(path_spec: &str) -> (String, FileAccessMode) {
    if let Some(pos) = path_spec.rfind(':') {
        let (path, mode_str) = path_spec.split_at(pos);
        let mode = &mode_str[1..];

        match mode {
            "ro" => (path.to_string(), FileAccessMode::ReadOnly),
            "rw" => (path.to_string(), FileAccessMode::ReadWrite),
            _ => (path_spec.to_string(), FileAccessMode::default()),
        }
    } else {
        (path_spec.to_string(), FileAccessMode::default())
    }
}

pub fn validate_path(
    path_spec: &str,
    project_root: &Path,
) -> Result<ParsedPath, PathValidationError> {
    let (path_str, mode) = parse_path_with_mode(path_spec);
    let p = Path::new(&path_str);

    if p.is_absolute() {
        return Err(PathValidationError::AbsolutePathNotAllowed(path_str));
    }

    let joined = project_root.join(p);
    let resolved = joined
        .canonicalize()
        .map_err(|_| PathValidationError::PathNotFound(path_str.clone()))?;

    let canonical_root = project_root
        .canonicalize()
        .map_err(|_| PathValidationError::EscapesProjectDirectory(path_str.clone()))?;

    if !resolved.starts_with(&canonical_root) {
        return Err(PathValidationError::EscapesProjectDirectory(path_str));
    }

    Ok(ParsedPath {
        path: resolved,
        guest_path: path_str,
        mode,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_absolute_path_rejected() {
        let temp = std::env::temp_dir();

        let result = validate_path("/absolute/path", &temp);
        assert!(matches!(
            result,
            Err(PathValidationError::AbsolutePathNotAllowed(_))
        ));
    }

    #[test]
    fn test_relative_path_works() {
        let current = std::env::current_dir().unwrap();

        let test_dir = current.join(".capsule_test");
        let _ = fs::create_dir(&test_dir);

        let result = validate_path("./.capsule_test", &current);

        let _ = fs::remove_dir(&test_dir);

        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.guest_path, "./.capsule_test");
    }

    #[test]
    fn test_non_existent_path_fails() {
        let current = std::env::current_dir().unwrap();

        let result = validate_path("./nonexistent_dir", &current);
        assert!(matches!(result, Err(PathValidationError::PathNotFound(_))));
    }

    #[test]
    fn test_escape_project_root_rejected() {
        let temp = std::env::temp_dir();
        let subdir = temp.join("test_subdir");
        let _ = fs::create_dir(&subdir);

        let result = validate_path("../", &subdir);

        let _ = fs::remove_dir(&subdir);

        assert!(matches!(
            result,
            Err(PathValidationError::EscapesProjectDirectory(_))
        ));
    }

    #[test]
    fn test_parse_mode_readonly() {
        let (path, mode) = parse_path_with_mode("./data:ro");
        assert_eq!(path, "./data");
        assert_eq!(mode, FileAccessMode::ReadOnly);
    }

    #[test]
    fn test_parse_mode_readwrite() {
        let (path, mode) = parse_path_with_mode("./output:rw");
        assert_eq!(path, "./output");
        assert_eq!(mode, FileAccessMode::ReadWrite);
    }

    #[test]
    fn test_parse_mode_default() {
        let (path, mode) = parse_path_with_mode("./data");
        assert_eq!(path, "./data");
        assert_eq!(mode, FileAccessMode::ReadWrite);
    }
}
