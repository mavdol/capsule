use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;

use super::CAPSULE_WIT;

pub enum PythonWasmCompilerError {
    CompileFailed(String),
    FsError(String),
}

impl fmt::Display for PythonWasmCompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PythonWasmCompilerError::CompileFailed(msg) => {
                write!(f, "Compilation failed > {}", msg)
            }
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
    fn get_python_command() -> String {
        let candidates = vec!["python3", "python"];

        for cmd in candidates {
            if Command::new(cmd)
                .arg("--version")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .is_ok()
            {
                return cmd.to_string();
            }
        }

        "python3".to_string()
    }

    fn normalize_path_for_command(path: &Path) -> PathBuf {
        #[cfg(windows)]
        {
            let path_str = path.to_string_lossy();
            if path_str.starts_with(r"\\?\") {
                return PathBuf::from(&path_str[4..]);
            }
        }
        path.to_path_buf()
    }

    pub fn new(source_path: &Path) -> Result<Self, PythonWasmCompilerError> {
        let source_path = source_path.canonicalize().map_err(|e| {
            PythonWasmCompilerError::FsError(format!("Cannot resolve source path: {}", e))
        })?;

        let source_dir = source_path
            .parent()
            .ok_or(PythonWasmCompilerError::FsError(
                "Cannot determine source directory".to_string(),
            ))?;

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
            let module_name = self
                .source_path
                .file_stem()
                .ok_or(PythonWasmCompilerError::FsError(
                    "Invalid source file name".to_string(),
                ))?
                .to_str()
                .ok_or(PythonWasmCompilerError::FsError(
                    "Invalid UTF-8 in file name".to_string(),
                ))?;

            let python_path = self
                .source_path
                .parent()
                .ok_or(PythonWasmCompilerError::FsError(
                    "Cannot determine parent directory".to_string(),
                ))?;

            let wit_path = self.get_wit_path()?;

            let sdk_path = self.get_sdk_path()?;

            if !sdk_path.exists() {
                return Err(PythonWasmCompilerError::FsError(format!(
                    "SDK directory not found: {}",
                    sdk_path.display()
                )));
            }

            if !sdk_path.exists() {
                return Err(PythonWasmCompilerError::FsError(format!(
                    "SDK directory not found: {}",
                    sdk_path.display()
                )));
            }

            let bootloader_path = self.cache_dir.join("_capsule_boot.py");
            let bootloader_content = format!(
                r#"# Auto-generated bootloader for Capsule
import {module_name}
import capsule.app
capsule.app._main_module = {module_name}
from capsule.app import TaskRunner, exports
"#,
                module_name = module_name
            );

            fs::write(&bootloader_path, bootloader_content)?;

            let wit_path_normalized = Self::normalize_path_for_command(&wit_path);
            let cache_dir_normalized = Self::normalize_path_for_command(&self.cache_dir);
            let python_path_normalized = Self::normalize_path_for_command(python_path);
            let sdk_path_normalized = Self::normalize_path_for_command(&sdk_path);
            let output_wasm_normalized = Self::normalize_path_for_command(&self.output_wasm);

            let output = Command::new("componentize-py")
                .arg("-d")
                .arg(&wit_path_normalized)
                .arg("-w")
                .arg("capsule-agent")
                .arg("componentize")
                .arg("_capsule_boot")
                .arg("-p")
                .arg(&cache_dir_normalized)
                .arg("-p")
                .arg(&python_path_normalized)
                .arg("-p")
                .arg(&sdk_path_normalized)
                .arg("-o")
                .arg(&output_wasm_normalized)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()?;

            if !output.status.success() {
                return Err(PythonWasmCompilerError::CompileFailed(format!(
                    "Compilation failed: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                )));
            }
        }

        Ok(self.output_wasm.clone())
    }

    fn needs_rebuild(
        &self,
        source: &Path,
        wasm_path: &Path,
    ) -> Result<bool, PythonWasmCompilerError> {
        if !wasm_path.exists() {
            return Ok(true);
        }

        let wasm_time = fs::metadata(wasm_path).and_then(|m| m.modified())?;

        let source_time = fs::metadata(source).and_then(|m| m.modified())?;
        if source_time > wasm_time {
            return Ok(true);
        }

        if let Some(source_dir) = source.parent()
            && Self::check_dir_modified(source_dir, source, wasm_time)?
        {
            return Ok(true);
        }

        Ok(false)
    }

    fn check_dir_modified(
        dir: &Path,
        source: &Path,
        wasm_time: std::time::SystemTime,
    ) -> Result<bool, PythonWasmCompilerError> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if dir_name.starts_with('.') || dir_name == "__pycache__" {
                        continue;
                    }

                    if Self::check_dir_modified(&path, source, wasm_time)? {
                        return Ok(true);
                    }
                } else if path.extension().is_some_and(|ext| ext == "py")
                    && path != source
                    && let Ok(metadata) = fs::metadata(&path)
                    && let Ok(modified) = metadata.modified()
                    && modified > wasm_time
                {
                    return Ok(true);
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
            let sdk_path = PathBuf::from(path);
            if sdk_path.exists() {
                return Ok(sdk_path);
            }
        }

        if let Ok(sdk_path) = self.find_sdk_via_python() {
            return Ok(sdk_path);
        }

        if let Ok(exe_path) = std::env::current_exe()
            && let Some(project_root) = exe_path
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

        Err(PythonWasmCompilerError::FsError(
            "Cannot find SDK. Set CAPSULE_SDK_PATH environment variable or install capsule package.".to_string(),
        ))
    }

    fn find_sdk_via_python(&self) -> Result<PathBuf, PythonWasmCompilerError> {
        let python_cmd = Self::get_python_command();
        let output = Command::new(&python_cmd)
            .arg("-c")
            .arg("import capsule; import os; print(os.path.dirname(os.path.dirname(capsule.__file__)), end='')")
            .output()
            .map_err(|e| {
                PythonWasmCompilerError::FsError(format!("Failed to execute {}: {}", python_cmd, e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PythonWasmCompilerError::FsError(format!(
                "Python command failed: {}",
                stderr
            )));
        }

        let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if path_str.is_empty() {
            return Err(PythonWasmCompilerError::FsError(
                "Python returned empty path for capsule package".to_string(),
            ));
        }

        let sdk_path = PathBuf::from(&path_str);

        if !sdk_path.exists() {
            return Err(PythonWasmCompilerError::FsError(format!(
                "SDK path from Python does not exist: {}",
                sdk_path.display()
            )));
        }

        Ok(sdk_path)
    }
}
