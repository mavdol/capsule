use std::fmt;
use std::path::Path;

use capsule_core::wasm::commands::create::CreateInstance;
use capsule_core::wasm::commands::run::RunInstance;
use capsule_core::wasm::execution_policy::ExecutionPolicy;
use capsule_core::wasm::compiler::python::{PythonWasmCompiler, PythonWasmCompilerError};
use capsule_core::wasm::runtime::WasmRuntimeError;
use capsule_core::wasm::runtime::Runtime;

pub enum RunError {
   IoError(String),
   CompileFailed(String),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::IoError(msg) => write!(f, "{}", msg),
            RunError::CompileFailed(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<std::io::Error> for RunError {
    fn from(err: std::io::Error) -> Self {
        RunError::IoError(err.to_string())
    }
}

impl From<PythonWasmCompilerError> for RunError {
    fn from(err: PythonWasmCompilerError) -> Self {
        RunError::CompileFailed(err.to_string())
    }
}

impl From<WasmRuntimeError> for RunError {
    fn from(err: WasmRuntimeError) -> Self {
        RunError::CompileFailed(err.to_string())
    }
}

pub async fn execute(file_path: &Path, args: Vec<String>) -> Result<String, RunError> {
    let compiler = PythonWasmCompiler::new(file_path)?;
    let wasm_path = compiler.compile_wasm()?;

    let runtime_config = capsule_core::wasm::runtime::RuntimeConfig {
        cache_dir: compiler.cache_dir.clone(),
    };
    let runtime = Runtime::with_config(runtime_config)?;

    let execution_policy = ExecutionPolicy::default();
    let create_instance_command = CreateInstance::new(execution_policy.clone(), args.clone())
        .wasm_path(wasm_path);

    let (store, instance, task_id) = runtime.execute(create_instance_command).await?;

    let args_json = serde_json::json!({
        "task_name": "main",
        "args": args,
        "kwargs": {}
    }).to_string();

    let run_instance_command = RunInstance::new(task_id, execution_policy, store, instance, args_json);

    let result = runtime.execute(run_instance_command).await?;

    Ok(result)
}
