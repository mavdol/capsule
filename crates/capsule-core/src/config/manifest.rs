use serde::Deserialize;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::wasm::execution_policy::Compute;

#[derive(Debug)]
pub enum ManifestError {
    FsError(String),
    ParseError(String),
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManifestError::FsError(e) => write!(f, "Filesystem error > {}", e),
            ManifestError::ParseError(e) => write!(f, "Parse error > {}", e),
        }
    }
}

impl std::error::Error for ManifestError {}

#[derive(Debug, Deserialize)]
pub struct Workflow {
    pub name: Option<String>,
    pub version: Option<String>,
    pub entrypoint: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DefaultPolicy {
    pub default_compute: Option<Compute>,
    pub default_ram: Option<String>,
    pub default_timeout: Option<String>,
    pub default_max_retries: Option<u64>,
    pub default_allowed_files: Option<Vec<String>>,
    pub default_env_variables: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
pub struct CapsuleToml {
    pub workflow: Option<Workflow>,
    pub tasks: Option<DefaultPolicy>,
}

pub struct Manifest {
    pub source_path: PathBuf,
    pub capsule_toml: CapsuleToml,
}

impl Manifest {
    pub fn new() -> Result<Self, ManifestError> {
        let source_path = Path::new(".")
            .canonicalize()
            .map_err(|e| ManifestError::FsError(format!("Cannot resolve source path: {}", e)))?;

        let capsule_toml_path = Self::find_capsule_toml(&source_path);

        let capsule_toml = match capsule_toml_path {
            Some(toml_path) => {
                let contents = fs::read_to_string(&toml_path).map_err(|e| {
                    ManifestError::FsError(format!("Failed to read capsule.toml: {}", e))
                })?;

                toml::from_str(&contents).map_err(|e| {
                    ManifestError::ParseError(format!("Failed to parse capsule.toml: {}", e))
                })?
            }
            None => CapsuleToml {
                workflow: None,
                tasks: None,
            },
        };

        Ok(Self {
            source_path,
            capsule_toml,
        })
    }

    pub fn get_entrypoint(&self) -> Result<String, ManifestError> {
        let workflow = self
        .capsule_toml
        .workflow
        .as_ref()
        .ok_or_else(|| {
            ManifestError::ParseError(
                "No [workflow] section found in capsule.toml. Please add a workflow configuration or specify a file path.".to_string()
            )
        })?;

        let entrypoint = workflow.entrypoint.as_ref().ok_or_else(|| {
            ManifestError::ParseError(
                "No 'entrypoint' field found in [workflow] section of capsule.toml.".to_string(),
            )
        })?;

        Ok(entrypoint.clone())
    }

    fn find_capsule_toml(dir: &Path) -> Option<PathBuf> {
        let lowercase = dir.join("capsule.toml");
        if lowercase.exists() {
            return Some(lowercase);
        }

        let capitalized = dir.join("Capsule.toml");
        if capitalized.exists() {
            return Some(capitalized);
        }

        None
    }
}
