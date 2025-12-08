use std::fmt;

use wasmtime::{Config, Engine};

use crate::config::log::{Log, LogError};

pub enum WasmRuntimeError {
    WasmtimeError(wasmtime::Error),
    LogError(LogError),
}

impl fmt::Display for WasmRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmRuntimeError::WasmtimeError(msg) => {
                write!(f, "Runtime error > Wasmtime error > {}", msg)
            }
            WasmRuntimeError::LogError(msg) => write!(f, "Runtime error > {}", msg),
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
    fn execute(self, runtime: &Runtime) -> impl Future<Output = Result<Self::Output, WasmRuntimeError>> + Send;
}

pub struct Runtime {
    pub(crate) engine: Engine,
    pub(crate) log: Log,
}

impl Runtime {
    pub fn new() -> Result<Self, WasmRuntimeError> {
        let mut config = Config::new();
        let log = Log::new(Some(".capsule"), "state.db-wal")?;

        config.wasm_component_model(true);
        config.async_support(true);
        config.consume_fuel(true);

        Ok(Self {
            engine: Engine::new(&config)?,
            log,
        })
    }

    pub async fn execute<C: RuntimeCommand>(
        &self,
        command: C,
    ) -> Result<C::Output, WasmRuntimeError> {
        command.execute(self).await
    }
}
