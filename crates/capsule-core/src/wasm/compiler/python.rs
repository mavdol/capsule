use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::CAPSULE_WIT;

pub enum PythonWasmCompilerError {
    CompileFailed(String),
    FsError(String),
}

impl fmt::Display for PythonWasmCompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PythonWasmCompilerError::CompileFailed(msg) => write!(f, "Compilation failed > {}", msg),
            PythonWasmCompilerError::FsError(msg) => write!(f, "File system error > {}", msg),
        }
    }
}

impl From<std::io::Error> for PythonWasmCompilerError {
    fn from(err: std::io::Error) -> Self {
        PythonWasmCompilerError::CompileFailed(err.to_string())
    }
}

impl From<std::time::SystemTimeError> for PythonWasmCompilerError {
    fn from(err: std::time::SystemTimeError) -> Self {
        PythonWasmCompilerError::FsError(err.to_string())
    }
}

pub struct PythonWasmCompiler {
    pub source_path: PathBuf,
    pub cache_dir: PathBuf,
    pub output_wasm: PathBuf,
}

impl PythonWasmCompiler {
    pub fn new(source_path: &Path) -> Result<Self, PythonWasmCompilerError> {
        let source_path = source_path.canonicalize()
            .map_err(|e| PythonWasmCompilerError::FsError(format!("Cannot resolve source path: {}", e)))?;

        let source_dir = source_path.parent()
            .ok_or(PythonWasmCompilerError::FsError("Cannot determine source directory".to_string()))?;

        let cache_dir = source_dir.join(".capsule");
        let output_wasm = cache_dir.join("capsule.wasm");

        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }

        Ok(Self {
            source_path,
            cache_dir,
            output_wasm,
        })
    }

    pub fn compile_wasm(&self) -> Result<PathBuf, PythonWasmCompilerError> {
        if self.needs_rebuild(&self.source_path, &self.output_wasm)? {
            let module_name = self.source_path
                .file_stem()
                .ok_or(PythonWasmCompilerError::FsError("Invalid source file name".to_string()))?
                .to_str()
                .ok_or(PythonWasmCompilerError::FsError("Invalid UTF-8 in file name".to_string()))?;

            let python_path = self.source_path
                .parent()
                .ok_or(PythonWasmCompilerError::FsError("Cannot determine parent directory".to_string()))?;

            let wit_path = self.get_wit_path()?;

            let sdk_path = self.get_sdk_path()?;

            if !sdk_path.exists() {
                return Err(PythonWasmCompilerError::FsError(
                    format!("SDK directory not found: {}", sdk_path.display())
                ));
            }

            if !sdk_path.exists() {
                return Err(PythonWasmCompilerError::FsError(
                    format!("SDK directory not found: {}", sdk_path.display())
                ));
            }

            let bootloader_path = self.cache_dir.join("_capsule_boot.py");
            let bootloader_content = format!(
                r#"# Auto-generated bootloader for Capsule
# This file imports the user's module and exports TaskRunner for componentize-py

# Import the user's module - this registers all @task decorated functions
import {module_name}

# Re-export the TaskRunner and exports from the SDK
# This is what componentize-py needs to find
from capsule.app import TaskRunner, exports
"#,
                module_name = module_name
            );

            fs::write(&bootloader_path, bootloader_content)?;

            let status = Command::new("componentize-py")
                .arg("-d")
                .arg(&wit_path)
                .arg("-w")
                .arg("capsule-agent")
                .arg("componentize")
                .arg("_capsule_boot")
                .arg("-p")
                .arg(&self.cache_dir)
                .arg("-p")
                .arg(python_path)
                .arg("-p")
                .arg(&sdk_path)
                .arg("-o")
                .arg(&self.output_wasm)
                .status()?;

            if !status.success() {
                return Err(PythonWasmCompilerError::CompileFailed(
                    "Compilation failed".to_string(),
                ));
            }
        }

        Ok(self.output_wasm.clone())
    }

    fn needs_rebuild(&self, source: &Path, wasm_path: &Path) -> Result<bool, PythonWasmCompilerError> {
        if !wasm_path.exists() {
            return Ok(true);
        }

        let wasm_time = fs::metadata(wasm_path).and_then(|m| m.modified())?;

        let source_time = fs::metadata(source).and_then(|m| m.modified())?;
        if source_time > wasm_time {
            return Ok(true);
        }

        if let Some(source_dir) = source.parent() {
            if let Ok(entries) = fs::read_dir(source_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "py") && path != source {
                        if let Ok(metadata) = fs::metadata(&path) {
                            if let Ok(modified) = metadata.modified() {
                                if modified > wasm_time {
                                    return Ok(true);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    fn get_wit_path(&self) -> Result<PathBuf, PythonWasmCompilerError> {
        if let Ok(path) = std::env::var("CAPSULE_WIT_PATH") {
            let wit_path = PathBuf::from(path);
            if wit_path.exists() {
                return Ok(wit_path);
            }
        }

        let wit_dir = self.cache_dir.join("wit");
        let wit_file = wit_dir.join("capsule.wit");

        if !wit_file.exists() {
            fs::create_dir_all(&wit_dir)?;
            fs::write(&wit_file, CAPSULE_WIT)?;
        }

        Ok(wit_dir)
    }

    fn get_sdk_path(&self) -> Result<PathBuf, PythonWasmCompilerError> {
        if let Ok(path) = std::env::var("CAPSULE_SDK_PATH") {
            return Ok(PathBuf::from(path));
        }

        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(project_root) = exe_path
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
            {
                let sdk_path = project_root.join("crates/capsule-sdk/python/src");
                if sdk_path.exists() {
                    return Ok(sdk_path);
                }
            }
        }

        Err(PythonWasmCompilerError::FsError(
            "Cannot find SDK. Set CAPSULE_SDK_PATH environment variable.".to_string()
        ))
    }
}
