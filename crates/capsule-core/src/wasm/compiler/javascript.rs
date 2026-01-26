use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::config::fingerprint::SourceFingerprint;
use crate::wasm::utilities::introspection::scanner;
use crate::wasm::utilities::wit_manager::WitManager;

#[derive(Debug)]
pub enum JavascriptWasmCompilerError {
    FsError(String),
    CommandFailed(String),
    CompileFailed(String),
}

impl fmt::Display for JavascriptWasmCompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    pub fn new(source_path: &Path) -> Result<Self, JavascriptWasmCompilerError> {
        let source_path = source_path.canonicalize().map_err(|e| {
            JavascriptWasmCompilerError::FsError(format!("Cannot resolve source path: {}", e))
        })?;

        let cache_dir = std::env::current_dir()
            .map_err(|e| {
                JavascriptWasmCompilerError::FsError(format!("Cannot get current directory: {}", e))
            })?
            .join(".capsule");

        fs::create_dir_all(&cache_dir)?;

        let output_wasm = cache_dir.join("capsule.wasm");

        Ok(Self {
            source_path,
            cache_dir,
            output_wasm,
        })
    }

    fn npx_command() -> Command {
        if Command::new("npx.cmd")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
        {
            Command::new("npx.cmd")
        } else {
            Command::new("npx")
        }
    }

    fn normalize_path_for_command(path: &Path) -> PathBuf {
        let path_str = path.to_string_lossy();
        if let Some(stripped) = path_str.strip_prefix(r"\\?\") {
            return PathBuf::from(stripped);
        }
        path.to_path_buf()
    }

    fn normalize_path_for_import(path: &Path) -> String {
        Self::normalize_path_for_command(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    pub fn compile_wasm(&self) -> Result<PathBuf, JavascriptWasmCompilerError> {
        let source_dir = self.source_path.parent().ok_or_else(|| {
            JavascriptWasmCompilerError::FsError("Cannot determine source directory".to_string())
        })?;

        if !SourceFingerprint::needs_rebuild(
            &self.cache_dir,
            source_dir,
            &self.output_wasm,
            &["js", "ts", "toml"],
            &["node_modules", "dist"],
        ) {
            return Ok(self.output_wasm.clone());
        }

        let wit_path = self.get_wit_path()?;

        let sdk_path = self.get_sdk_path()?;

        let source_for_import = if self.source_path.extension().is_some_and(|ext| ext == "ts") {
            self.transpile_typescript()?
        } else {
            self.source_path.clone()
        };

        let wrapper_path = self.cache_dir.join("_capsule_boot.js");
        let bundled_path = self.cache_dir.join("_capsule_bundled.js");

        let import_path = Self::normalize_path_for_import(
            &source_for_import
                .canonicalize()
                .unwrap_or_else(|_| source_for_import.to_path_buf()),
        );

        let sdk_path_str = Self::normalize_path_for_import(&sdk_path);

        let wrapper_content = format!(
            r#"// Auto-generated bootloader for Capsule
import * as hostApi from 'capsule:host/api';
import * as fsTypes from 'wasi:filesystem/types@0.2.0';
import * as fsPreopens from 'wasi:filesystem/preopens@0.2.0';
import * as environment from 'wasi:cli/environment@0.2.0';
globalThis['capsule:host/api'] = hostApi;
globalThis['wasi:filesystem/types'] = fsTypes;
globalThis['wasi:filesystem/preopens'] = fsPreopens;
globalThis['wasi:cli/environment'] = environment;
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

        let esbuild_output = Self::npx_command()
            .arg("esbuild")
            .arg(&wrapper_path_normalized)
            .arg("--bundle")
            .arg("--format=esm")
            .arg("--platform=neutral")
            .arg("--external:capsule:host/api")
            .arg("--external:wasi:filesystem/*")
            .arg("--external:wasi:cli/*")
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

        let jco_output = Self::npx_command()
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

        let _ = SourceFingerprint::update_after_build(
            &self.cache_dir,
            source_dir,
            &["js", "ts", "toml"],
            &["node_modules", "dist"],
        );

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

        if !wit_dir.join("capsule.wit").exists() {
            WitManager::import_wit_deps(&wit_dir)?;
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
            "Could not find JavaScript SDK.".to_string(),
        ))
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

        let output = Self::npx_command()
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

    pub fn introspect_task_registry(&self) -> Option<HashMap<String, serde_json::Value>> {
        let source_dir = self.source_path.parent()?;
        scanner::scan_js_tasks(source_dir)
    }
}
