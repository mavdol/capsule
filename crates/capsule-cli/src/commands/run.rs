use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Instant;

use capsule_core::config::manifest::{CapsuleToml, Manifest, ManifestError};

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
use capsule_core::wasm::utilities::task_config::TaskConfig;
use capsule_core::wasm::utilities::task_reporter::TaskReporter;

pub enum RunError {
    IoError(String),
    ExecutionFailed(String),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::IoError(msg) => write!(f, "{}", msg),
            RunError::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
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
        RunError::ExecutionFailed(err.to_string())
    }
}

impl From<JavascriptWasmCompilerError> for RunError {
    fn from(err: JavascriptWasmCompilerError) -> Self {
        RunError::ExecutionFailed(err.to_string())
    }
}

impl From<WasmRuntimeError> for RunError {
    fn from(err: WasmRuntimeError) -> Self {
        RunError::ExecutionFailed(err.to_string())
    }
}

impl From<ManifestError> for RunError {
    fn from(err: ManifestError) -> Self {
        RunError::ExecutionFailed(err.to_string())
    }
}

type TaskRegistry = HashMap<String, serde_json::Value>;

struct CompileResult {
    wasm_path: PathBuf,
    cache_dir: PathBuf,
    task_registry: Option<TaskRegistry>,
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
            let task_registry = compiler.introspect_task_registry();

            Ok(CompileResult {
                wasm_path,
                cache_dir: compiler.cache_dir,
                task_registry,
            })
        }
        "js" | "mjs" | "ts" => {
            let compiler = JavascriptWasmCompiler::new(file_path)?;
            let wasm_path = compiler.compile_wasm()?;
            let task_registry = compiler.introspect_task_registry();

            Ok(CompileResult {
                wasm_path,
                cache_dir: compiler.cache_dir,
                task_registry,
            })
        }
        _ => Err(RunError::ExecutionFailed(format!(
            "Unsupported file extension: '{}'. Supported: .py, .js, .mjs, .ts",
            extension
        ))),
    }
}

fn extract_main_execution_policy(
    task_registry: Option<TaskRegistry>,
    capsule_toml: &CapsuleToml,
) -> Option<ExecutionPolicy> {
    let registry = task_registry?;
    let main_config = registry.get("main")?;
    let task_config: TaskConfig = serde_json::from_value(main_config.clone()).ok()?;

    Some(task_config.to_execution_policy(capsule_toml))
}

fn load_env_variables(project_root: &Path) -> Result<(), RunError> {
    let env_files = vec![".env", ".env.local", ".env.development", ".env.production"];

    for file_name in env_files {
        let file_path = project_root.join(file_name);

        if file_path.exists() {
            dotenvy::from_path(&file_path).map_err(|e| RunError::IoError(e.to_string()))?;
        }
    }

    Ok(())
}

pub async fn execute(
    file_path: Option<&Path>,
    args: Vec<String>,
    verbose: bool,
) -> Result<String, RunError> {
    let manifest = Manifest::new()?;
    let mut reporter = TaskReporter::new(true);

    let file_path: PathBuf = match file_path {
        Some(path) => path.to_path_buf(),
        None => PathBuf::from(manifest.get_entrypoint()?),
    };

    reporter.start_progress("Preparing environment");
    let compile_result = compile_to_wasm(&file_path)?;
    reporter.finish_progress(Some("Environment ready"));

    reporter.start_progress("Initializing runtime");

    let project_root = file_path
        .canonicalize()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    load_env_variables(&project_root)?;

    let runtime_config = RuntimeConfig {
        cache_dir: compile_result.cache_dir,
        verbose,
    };

    let execution_policy =
        extract_main_execution_policy(compile_result.task_registry, &manifest.capsule_toml)
            .unwrap_or_else(|| ExecutionPolicy::default().compute(Some(Compute::Custom(u64::MAX))));

    let runtime = Runtime::new(runtime_config, manifest.capsule_toml)?;

    let create_instance_command = CreateInstance::new(execution_policy.clone(), args.clone())
        .wasm_path(compile_result.wasm_path)
        .project_root(project_root);

    let (store, instance, task_id) = runtime.execute(create_instance_command).await?;
    reporter.finish_progress(Some("Runtime ready"));

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
