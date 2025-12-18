use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;

use super::CAPSULE_WIT;

pub enum JavascriptWasmCompilerError {
    CompileFailed(String),
    FsError(String),
}

impl fmt::Display for JavascriptWasmCompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JavascriptWasmCompilerError::CompileFailed(msg) => {
                write!(f, "Compilation failed > {}", msg)
            }
            JavascriptWasmCompilerError::FsError(msg) => write!(f, "File system error > {}", msg),
        }
    }
}

impl From<std::io::Error> for JavascriptWasmCompilerError {
    fn from(err: std::io::Error) -> Self {
        JavascriptWasmCompilerError::CompileFailed(err.to_string())
    }
}

impl From<std::time::SystemTimeError> for JavascriptWasmCompilerError {
    fn from(err: std::time::SystemTimeError) -> Self {
        JavascriptWasmCompilerError::FsError(err.to_string())
    }
}

pub struct JavascriptWasmCompiler {
    pub source_path: PathBuf,
    pub cache_dir: PathBuf,
    pub output_wasm: PathBuf,
}

impl JavascriptWasmCompiler {
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

    pub fn new(source_path: &Path) -> Result<Self, JavascriptWasmCompilerError> {
        let source_path = source_path.canonicalize().map_err(|e| {
            JavascriptWasmCompilerError::FsError(format!("Cannot resolve source path: {}", e))
        })?;

        let source_dir = source_path
            .parent()
            .ok_or(JavascriptWasmCompilerError::FsError(
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

    pub fn compile_wasm(&self) -> Result<PathBuf, JavascriptWasmCompilerError> {
        if self.needs_rebuild(&self.source_path, &self.output_wasm)? {
            let wit_path = self.get_wit_path()?;

            let sdk_path = self.get_sdk_path()?;

            let source_for_import = if self.source_path.extension().is_some_and(|ext| ext == "ts") {
                self.transpile_typescript()?
            } else {
                self.source_path.clone()
            };

            let wrapper_path = self.cache_dir.join("_capsule_boot.js");
            let source_dir = self
                .source_path
                .parent()
                .ok_or(JavascriptWasmCompilerError::FsError(
                    "Cannot determine parent directory".to_string(),
                ))?;

            let module_name = source_for_import
                .file_stem()
                .ok_or(JavascriptWasmCompilerError::FsError(
                    "Invalid source file name".to_string(),
                ))?
                .to_str()
                .ok_or(JavascriptWasmCompilerError::FsError(
                    "Invalid UTF-8 in file name".to_string(),
                ))?;


            let wrapper_content = format!(
                r#"// Auto-generated bootloader for Capsule

// Import user module and SDK
import './{module_name}.js';
import {{ exports }} from '{sdk_path}/capsule/app.js';

// Re-export the TaskRunner interface
export {{ exports }};
"#,
                module_name = module_name,
                sdk_path = sdk_path.display()
            );

            fs::write(&wrapper_path, wrapper_content)?;

            let wit_path_normalized = Self::normalize_path_for_command(&wit_path);
            let wrapper_path_normalized = Self::normalize_path_for_command(&wrapper_path);
            let output_wasm_normalized = Self::normalize_path_for_command(&self.output_wasm);

            let output = Command::new("jco")
                .arg("componentize")
                .arg(&wrapper_path_normalized)
                .arg("--wit")
                .arg(&wit_path_normalized)
                .arg("-n")
                .arg("capsule-agent")
                .arg("-o")
                .arg(&output_wasm_normalized)
                .current_dir(source_dir)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()?;

            if !output.status.success() {
                return Err(JavascriptWasmCompilerError::CompileFailed(format!(
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
    ) -> Result<bool, JavascriptWasmCompilerError> {
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
    ) -> Result<bool, JavascriptWasmCompilerError> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if dir_name.starts_with('.') || dir_name == "node_modules" {
                        continue;
                    }

                    if Self::check_dir_modified(&path, source, wasm_time)? {
                        return Ok(true);
                    }
                } else if path.extension().is_some_and(|ext| ext == "js" || ext == "mjs" || ext == "ts")
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

    fn get_wit_path(&self) -> Result<PathBuf, JavascriptWasmCompilerError> {
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

    fn get_sdk_path(&self) -> Result<PathBuf, JavascriptWasmCompilerError> {
        if let Ok(path) = std::env::var("CAPSULE_JS_SDK_PATH") {
            let sdk_path = PathBuf::from(path);
            if sdk_path.exists() {
                return Ok(sdk_path);
            }
        }

        if let Ok(sdk_path) = self.find_sdk_via_npm() {
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
            let sdk_path = project_root.join("crates/capsule-sdk/javascript/src");
            if sdk_path.exists() {
                return Ok(sdk_path);
            }
        }

        Err(JavascriptWasmCompilerError::FsError(
            "Cannot find Javascript SDK. Set CAPSULE_JS_SDK_PATH environment variable or install @capsule/sdk package.".to_string(),
        ))
    }

    fn find_sdk_via_npm(&self) -> Result<PathBuf, JavascriptWasmCompilerError> {
        let output = Command::new("npm")
            .arg("root")
            .output()
            .map_err(|e| {
                JavascriptWasmCompilerError::FsError(format!("Failed to execute npm: {}", e))
            })?;

        if !output.status.success() {
            return Err(JavascriptWasmCompilerError::FsError(
                "npm root command failed".to_string(),
            ));
        }

        let node_modules = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if node_modules.is_empty() {
            return Err(JavascriptWasmCompilerError::FsError(
                "npm returned empty path".to_string(),
            ));
        }

        let sdk_path = PathBuf::from(&node_modules).join("@capsule/sdk/src");

        if !sdk_path.exists() {
            return Err(JavascriptWasmCompilerError::FsError(format!(
                "SDK path from npm does not exist: {}",
                sdk_path.display()
            )));
        }

        Ok(sdk_path)
    }

    fn transpile_typescript(&self) -> Result<PathBuf, JavascriptWasmCompilerError> {
        let module_name = self
            .source_path
            .file_stem()
            .ok_or(JavascriptWasmCompilerError::FsError(
                "Invalid source file name".to_string(),
            ))?
            .to_str()
            .ok_or(JavascriptWasmCompilerError::FsError(
                "Invalid UTF-8 in file name".to_string(),
            ))?;

        let output_path = self.cache_dir.join(format!("{}.js", module_name));

        let output = Command::new("tsc")
            .arg(&self.source_path)
            .arg("--outDir")
            .arg(&self.cache_dir)
            .arg("--module")
            .arg("esnext")
            .arg("--target")
            .arg("esnext")
            .arg("--moduleResolution")
            .arg("node")
            .arg("--esModuleInterop")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            return Err(JavascriptWasmCompilerError::CompileFailed(format!(
                "TypeScript compilation failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            )));
        }

        if !output_path.exists() {
            return Err(JavascriptWasmCompilerError::FsError(format!(
                "TypeScript transpilation did not produce expected output: {}",
                output_path.display()
            )));
        }

        Ok(output_path)
    }

}
