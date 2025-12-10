use std::fmt;
use std::path::Path;
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use capsule_core::wasm::commands::create::CreateInstance;
use capsule_core::wasm::commands::run::RunInstance;
use capsule_core::wasm::compiler::python::{PythonWasmCompiler, PythonWasmCompilerError};
use capsule_core::wasm::execution_policy::{Compute, ExecutionPolicy};
use capsule_core::wasm::runtime::Runtime;
use capsule_core::wasm::runtime::WasmRuntimeError;

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

impl From<WasmRuntimeError> for RunError {
    fn from(err: WasmRuntimeError) -> Self {
        RunError::CompileFailed(err.to_string())
    }
}

pub async fn execute(file_path: &Path, args: Vec<String>) -> Result<String, RunError> {
    let compiler = PythonWasmCompiler::new(file_path)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    spinner.set_message("Preparing capsule environment");

    let wasm_path = compiler.compile_wasm()?;

    spinner.finish_and_clear();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    spinner.set_message("Launching capsule runtime");
    let runtime_config = capsule_core::wasm::runtime::RuntimeConfig {
        cache_dir: compiler.cache_dir.clone(),
    };
    let runtime = Runtime::with_config(runtime_config)?;

    let execution_policy = ExecutionPolicy::default().compute(Some(Compute::High));
    let create_instance_command =
        CreateInstance::new(execution_policy.clone(), args.clone()).wasm_path(wasm_path);

    let (store, instance, task_id) = runtime.execute(create_instance_command).await?;
    spinner.finish_and_clear();

    println!("📡 Capsule in orbit. Systems nominal.");

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
    let time_str = format_duration(elapsed);
    println!("✓ Complete ({})", time_str);

    Ok(result)
}

fn format_duration(duration: std::time::Duration) -> String {
    let total_secs = duration.as_secs_f64();

    if total_secs < 60.0 {
        format!("{:.2}s", total_secs)
    } else if total_secs < 3600.0 {
        let minutes = (total_secs / 60.0).floor() as u64;
        let seconds = total_secs % 60.0;
        format!("{}m {:.0}s", minutes, seconds)
    } else {
        let hours = (total_secs / 3600.0).floor() as u64;
        let remaining_secs = total_secs % 3600.0;
        let minutes = (remaining_secs / 60.0).floor() as u64;
        let seconds = remaining_secs % 60.0;
        format!("{}h {}m {:.0}s", hours, minutes, seconds)
    }
}
