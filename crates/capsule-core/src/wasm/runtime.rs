use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::log::{Log, LogError};
use wasmtime::component::Component;
use wasmtime::{Config, Engine};

pub enum WasmRuntimeError {
    WasmtimeError(wasmtime::Error),
    LogError(LogError),
    ConfigError(String),
    Timeout(String),
}

impl fmt::Display for WasmRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmRuntimeError::WasmtimeError(msg) => {
                write!(f, "Runtime error > Wasmtime error > {}", msg)
            }
            WasmRuntimeError::LogError(msg) => write!(f, "Runtime error > {}", msg),
            WasmRuntimeError::ConfigError(msg) => write!(f, "Runtime error > Config > {}", msg),
            WasmRuntimeError::Timeout(task_id) => write!(f, "Task '{}' timed out", task_id),
        }
    }
}

impl From<wasmtime::Error> for WasmRuntimeError {
    fn from(err: wasmtime::Error) -> Self {
        WasmRuntimeError::WasmtimeError(err)
    }
}

impl From<LogError> for WasmRuntimeError {
    fn from(err: LogError) -> Self {
        WasmRuntimeError::LogError(err)
    }
}

pub trait RuntimeCommand {
    type Output;
    fn execute(
        self,
        runtime: Arc<Runtime>,
    ) -> impl Future<Output = Result<Self::Output, WasmRuntimeError>> + Send;
}

pub struct RuntimeConfig {
    pub cache_dir: PathBuf,
    pub verbose: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from(".capsule"),
            verbose: false,
        }
    }
}

pub struct Runtime {
    pub(crate) engine: Engine,
    pub(crate) log: Log,

    #[allow(dead_code)]
    pub(crate) cache_dir: PathBuf,

    pub verbose: bool,

    component: RwLock<Option<Component>>,
}

impl Runtime {
    pub fn new() -> Result<Arc<Self>, WasmRuntimeError> {
        Self::with_config(RuntimeConfig::default())
    }

    pub fn with_config(config: RuntimeConfig) -> Result<Arc<Self>, WasmRuntimeError> {
        let mut engine_config = Config::new();
        let db_path = config.cache_dir.join("state.db");
        let log = Log::new(
            Some(db_path.parent().unwrap().to_str().unwrap()),
            db_path.file_name().unwrap().to_str().unwrap(),
        )?;

        engine_config.wasm_component_model(true);
        engine_config.async_support(true);
        engine_config.consume_fuel(true);

        Ok(Arc::new(Self {
            engine: Engine::new(&engine_config)?,
            log,
            cache_dir: config.cache_dir,
            verbose: config.verbose,
            component: RwLock::new(None),
        }))
    }

    pub async fn execute<C: RuntimeCommand>(
        self: &Arc<Self>,
        command: C,
    ) -> Result<C::Output, WasmRuntimeError> {
        command.execute(Arc::clone(self)).await
    }

    pub async fn get_component(&self) -> Option<Component> {
        self.component.read().await.clone()
    }

    pub async fn set_component(&self, component: Component) {
        *self.component.write().await = Some(component);
    }
}
