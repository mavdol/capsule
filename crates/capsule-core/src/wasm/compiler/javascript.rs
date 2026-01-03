use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::CAPSULE_WIT;

#[derive(Debug)]
pub enum JavascriptWasmCompilerError {
    FsError(String),
    CommandFailed(String),
    CompileFailed(String),
}

impl std::fmt::Display for JavascriptWasmCompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JavascriptWasmCompilerError::FsError(msg) => write!(f, "Filesystem error > {}", msg),
            JavascriptWasmCompilerError::CommandFailed(msg) => {
                write!(f, "Command failed > {}", msg)
            }
            JavascriptWasmCompilerError::CompileFailed(msg) => {
                write!(f, "Compilation failed > {}", msg)
            }
        }
    }
}

impl From<std::io::Error> for JavascriptWasmCompilerError {
    fn from(err: std::io::Error) -> Self {
        JavascriptWasmCompilerError::FsError(err.to_string())
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

        let cache_dir = source_path
            .parent()
            .ok_or_else(|| JavascriptWasmCompilerError::FsError("Invalid source path".to_string()))?
            .join(".capsule");

        fs::create_dir_all(&cache_dir)?;

        let output_wasm = cache_dir.join("agent.wasm");

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
            let bundled_path = self.cache_dir.join("_capsule_bundled.js");

            let import_path = source_for_import
                .canonicalize()
                .unwrap_or_else(|_| source_for_import.to_path_buf())
                .display()
                .to_string();

            let sdk_path_str = sdk_path.to_str().ok_or_else(|| {
                JavascriptWasmCompilerError::FsError("Invalid SDK path".to_string())
            })?;

            let wrapper_content = format!(
                r#"// Auto-generated bootloader for Capsule
                    import * as hostApi from 'capsule:host/api';
                    globalThis['capsule:host/api'] = hostApi;
                    import '{}';
                    import {{ exports }} from '{}/dist/app.js';
                    export const taskRunner = exports;
                "#,
                import_path, sdk_path_str
            );

            fs::write(&wrapper_path, wrapper_content)?;

            let wrapper_path_normalized = Self::normalize_path_for_command(&wrapper_path);
            let bundled_path_normalized = Self::normalize_path_for_command(&bundled_path);
            let wit_path_normalized = Self::normalize_path_for_command(&wit_path);
            let sdk_path_normalized = Self::normalize_path_for_command(&sdk_path);
            let output_wasm_normalized = Self::normalize_path_for_command(&self.output_wasm);

            let esbuild_output = Command::new("npx")
                .arg("esbuild")
                .arg(&wrapper_path_normalized)
                .arg("--bundle")
                .arg("--format=esm")
                .arg("--platform=neutral")
                .arg("--external:capsule:host/api")
                .arg(format!("--outfile={}", bundled_path_normalized.display()))
                .current_dir(&sdk_path_normalized)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()?;

            if !esbuild_output.status.success() {
                return Err(JavascriptWasmCompilerError::CompileFailed(format!(
                    "Bundling failed: {}",
                    String::from_utf8_lossy(&esbuild_output.stderr).trim()
                )));
            }

            let jco_output = Command::new("npx")
                .arg("jco")
                .arg("componentize")
                .arg(&bundled_path_normalized)
                .arg("--wit")
                .arg(&wit_path_normalized)
                .arg("--world-name")
                .arg("capsule-agent")
                .arg("--enable")
                .arg("http")
                .arg("-o")
                .arg(&output_wasm_normalized)
                .current_dir(&sdk_path_normalized)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()?;

            if !jco_output.status.success() {
                return Err(JavascriptWasmCompilerError::CompileFailed(format!(
                    "Component creation failed: {}",
                    String::from_utf8_lossy(&jco_output.stderr).trim()
                )));
            }
        }

        Ok(self.output_wasm.clone())
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

        if let Some(source_dir) = self.source_path.parent() {
            let node_modules_sdk = source_dir.join("node_modules/@capsule-run/sdk");
            if node_modules_sdk.exists() {
                return Ok(node_modules_sdk);
            }
        }

        if let Ok(exe_path) = std::env::current_exe()
            && let Some(project_root) = exe_path
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
        {
            let sdk_path = project_root.join("crates/capsule-sdk/javascript");
            if sdk_path.exists() {
                return Ok(sdk_path);
            }
        }

        Err(JavascriptWasmCompilerError::FsError(
            "Could not find JavaScript SDK. Set CAPSULE_JS_SDK_PATH environment variable or run 'npm link @capsule-run/sdk' in your project directory.".to_string()
        ))
    }

    fn needs_rebuild(
        &self,
        source: &Path,
        output: &Path,
    ) -> Result<bool, JavascriptWasmCompilerError> {
        if !output.exists() {
            return Ok(true);
        }

        let output_time = fs::metadata(output).and_then(|m| m.modified())?;

        let source_time = fs::metadata(source).and_then(|m| m.modified())?;
        if source_time > output_time {
            return Ok(true);
        }

        if let Some(source_dir) = source.parent()
            && Self::check_dir_modified(source_dir, source, output_time)?
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
                } else if let Some(ext) = path.extension()
                    && (ext == "js" || ext == "ts")
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

    fn transpile_typescript(&self) -> Result<PathBuf, JavascriptWasmCompilerError> {
        let output_path = self.cache_dir.join(
            self.source_path
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| format!("{}.js", s))
                .ok_or_else(|| {
                    JavascriptWasmCompilerError::FsError("Invalid source filename".to_string())
                })?,
        );

        let output = Command::new("npx")
            .arg("tsc")
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
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("TypeScript compilation failed!");
            eprintln!("stdout: {}", stdout);
            eprintln!("stderr: {}", stderr);
            return Err(JavascriptWasmCompilerError::CompileFailed(format!(
                "TypeScript compilation failed: {}{}",
                stderr.trim(),
                if !stdout.is_empty() {
                    format!("\nstdout: {}", stdout.trim())
                } else {
                    String::new()
                }
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
