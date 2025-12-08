use std::fmt;
use std::fs;
use std::path::Path;
use std::process::Command;

pub enum PythonWasmCompilerError {
    CompileFailed(String),
}

impl fmt::Display for PythonWasmCompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PythonWasmCompilerError::CompileFailed(msg) => write!(f, "Compilation failed: {}", msg),
        }
    }
}

impl From<std::io::Error> for PythonWasmCompilerError {
    fn from(err: std::io::Error) -> Self {
        PythonWasmCompilerError::CompileFailed(err.to_string())
    }
}

pub struct PythonWasmCompiler {
    pub cache_dir: String,
    pub output_wasm: String,
}

impl PythonWasmCompiler {
    pub fn new() -> Result<Self, PythonWasmCompilerError> {
        let cache_dir = ".capsule".to_string();
        let output_wasm = format!("{}/capsule.wasm", &cache_dir);

        if !Path::new(&cache_dir).exists() {
            fs::create_dir_all(&cache_dir)?;
        }

        Ok(Self {
            cache_dir,
            output_wasm,
        })
    }

    pub fn compile_wasm(&self, source: &str) -> Result<(), PythonWasmCompilerError> {
        if !Path::new(&self.cache_dir).exists() {
            fs::create_dir_all(&self.cache_dir)?;
        }

        if self.needs_rebuild(source, &self.output_wasm) {
            let status = Command::new("componentize-py")
                .arg("-d")
                .arg(".")
                .arg("-m")
                .arg(source)
                .arg("-o")
                .arg(&self.output_wasm)
                .status()?;

            if !status.success() {
                return Err(PythonWasmCompilerError::CompileFailed(
                    "Compilation failed".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn needs_rebuild(&self, source: &str, wasm_path: &str) -> bool {
        let source_time = fs::metadata(source).and_then(|m| m.modified()).unwrap();
        let wasm_time = fs::metadata(wasm_path).and_then(|m| m.modified()).unwrap();

        if source_time > wasm_time {
            return true;
        }

        false
    }
}
