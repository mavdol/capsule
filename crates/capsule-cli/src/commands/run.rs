use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Instant;

use capsule_core::wasm::commands::create::CreateInstance;
use capsule_core::wasm::commands::run::RunInstance;
use capsule_core::wasm::compiler::javascript::{
    JavascriptWasmCompiler, JavascriptWasmCompilerError,
};
use capsule_core::wasm::compiler::python::{PythonWasmCompiler, PythonWasmCompilerError};
use capsule_core::wasm::execution_policy::{Compute, ExecutionPolicy};
use capsule_core::wasm::runtime::Runtime;
use capsule_core::wasm::runtime::RuntimeConfig;
use capsule_core::wasm::runtime::WasmRuntimeError;
use capsule_core::wasm::utilities::task_reporter::TaskReporter;

pub enum RunError {
    IoError(String),
    CompileFailed(String),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::IoError(msg) => write!(f, "{}", msg),
            RunError::CompileFailed(msg) => write!(f, "🛠️ Mission aborted: {}", msg),
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

impl From<JavascriptWasmCompilerError> for RunError {
    fn from(err: JavascriptWasmCompilerError) -> Self {
        RunError::CompileFailed(err.to_string())
    }
}

impl From<WasmRuntimeError> for RunError {
    fn from(err: WasmRuntimeError) -> Self {
        RunError::CompileFailed(err.to_string())
    }
}

struct CompileResult {
    wasm_path: PathBuf,
    cache_dir: PathBuf,
}

fn compile_to_wasm(file_path: &Path) -> Result<CompileResult, RunError> {
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension {
        "py" => {
            let compiler = PythonWasmCompiler::new(file_path)?;
            let wasm_path = compiler.compile_wasm()?;
            Ok(CompileResult {
                wasm_path,
                cache_dir: compiler.cache_dir,
            })
        }
        "js" | "mjs" | "ts" => {
            let compiler = JavascriptWasmCompiler::new(file_path)?;
            let wasm_path = compiler.compile_wasm()?;
            Ok(CompileResult {
                wasm_path,
                cache_dir: compiler.cache_dir,
            })
        }
        _ => Err(RunError::CompileFailed(format!(
            "Unsupported file extension: '{}'. Supported: .py, .js, .mjs, .ts",
            extension
        ))),
    }
}

pub async fn execute(
    file_path: &Path,
    args: Vec<String>,
    verbose: bool,
) -> Result<String, RunError> {
    let mut reporter = TaskReporter::new(true);

    reporter.start_progress("Preparing capsule environment");
    let compile_result = compile_to_wasm(file_path)?;
    reporter.finish_progress(Some("Environment prepared"));

    reporter.start_progress("Launching capsule runtime");
    let runtime_config = RuntimeConfig {
        cache_dir: compile_result.cache_dir,
        verbose,
    };
    let runtime = Runtime::with_config(runtime_config)?;

    let execution_policy = ExecutionPolicy::default().compute(Some(Compute::Custom(u64::MAX)));
    let create_instance_command = CreateInstance::new(execution_policy.clone(), args.clone())
        .wasm_path(compile_result.wasm_path);

    let (store, instance, task_id) = runtime.execute(create_instance_command).await?;
    reporter.finish_progress(Some("Runtime launched"));

    reporter.success("📡 Capsule in orbit. Systems nominal.");

    let start_time = Instant::now();

    let args_json = serde_json::json!({
        "task_name": "main",
        "args": args,
        "kwargs": {}
    })
    .to_string();

    let run_instance_command =
        RunInstance::new(task_id, execution_policy, store, instance, args_json);

    let result = runtime.execute(run_instance_command).await?;

    let elapsed = start_time.elapsed();
    let time_str = reporter.format_duration(elapsed);
    reporter.success(&format!("✓ Complete ({})", time_str));

    Ok(result)
}
