use std::sync::Arc;

use anyhow::Result;
use wasmtime::component::{ResourceTable, bindgen};
use wasmtime::{ResourceLimiter, StoreLimits};
use wasmtime_wasi::{WasiCtx, WasiView};

use crate::wasm::commands::create::CreateInstance;
use crate::wasm::commands::run::RunInstance;
use crate::wasm::utilities::task_config::TaskConfig;
use capsule::host::api::{Host, TaskError};

bindgen!({
    path: "../capsule-wit",
    world: "capsule-agent",
    async: true,
});

pub use capsule::host::api as host_api;

pub struct State {
    pub ctx: WasiCtx,
    pub table: ResourceTable,
    pub limits: StoreLimits,
    pub runtime: Option<Arc<crate::wasm::runtime::Runtime>>,
}

impl WasiView for State {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl Host for State {
    async fn schedule_task(
        &mut self,
        name: String,
        args: String,
        config: String,
    ) -> Result<String, TaskError> {
        println!("📞 [Host] schedule_task: {} (Config: {})", name, config);

        let runtime = match &self.runtime {
            Some(r) => Arc::clone(r),
            None => {
                return Err(TaskError::InternalError(
                    "No runtime available for recursive task execution".to_string(),
                ));
            }
        };

        let task_config: TaskConfig = serde_json::from_str(&config).unwrap_or_default();
        let policy = task_config.to_execution_policy();

        let create_cmd = CreateInstance::new(policy.clone(), vec![]).task_name(&name);

        let (store, instance, task_id) = runtime
            .execute(create_cmd)
            .await
            .map_err(|e| TaskError::InternalError(format!("Failed to create instance: {}", e)))?;

        let args_json = format!(
            r#"{{"task_name": "{}", "args": {}, "kwargs": {{}}}}"#,
            name, args
        );

        let run_cmd = RunInstance::new(task_id, policy, store, instance, args_json);

        let result = runtime
            .execute(run_cmd)
            .await
            .map_err(|e| TaskError::InternalError(format!("Failed to run instance: {}", e)))?;

        Ok(result)
    }
}

impl ResourceLimiter for State {
    fn memory_growing(
        &mut self,
        current: usize,
        desired: usize,
        maximum: Option<usize>,
    ) -> Result<bool> {
        self.limits.memory_growing(current, desired, maximum)
    }

    fn table_growing(
        &mut self,
        current: usize,
        desired: usize,
        maximum: Option<usize>,
    ) -> Result<bool> {
        self.limits.table_growing(current, desired, maximum)
    }
}
