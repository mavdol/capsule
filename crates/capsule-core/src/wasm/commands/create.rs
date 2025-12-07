use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Store, StoreLimitsBuilder};
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::p2::add_to_linker_async;

use crate::config::log::{CreateInstanceLog, InstanceState};
use crate::wasm::execution_policy::ExecutionPolicy;
use crate::wasm::runtime::{Runtime, RuntimeCommand, WasmRuntimeError};
use crate::wasm::state::WasmState;

pub struct CreateInstance {
    policy: ExecutionPolicy,
    args: Vec<String>,
    task_id: String,
    task_name: String,
    agent_name: String,
    agent_version: String,
}

impl CreateInstance {
    pub fn new(policy: ExecutionPolicy, args: Vec<String>) -> Self {
        Self {
            policy,
            args,
            task_id: "default_task".to_string(),
            task_name: "default_task_name".to_string(),
            agent_name: "default_agent".to_string(),
            agent_version: "0.0.0".to_string(),
        }
    }

    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = task_id.into();
        self
    }

    pub fn task_name(mut self, task_name: impl Into<String>) -> Self {
        self.task_name = task_name.into();
        self
    }

    pub fn agent_name(mut self, agent_name: impl Into<String>) -> Self {
        self.agent_name = agent_name.into();
        self
    }

    pub fn agent_version(mut self, agent_version: impl Into<String>) -> Self {
        self.agent_version = agent_version.into();
        self
    }
}

impl RuntimeCommand for CreateInstance {
    type Output = bool;

    async fn execute(self, runtime: &Runtime) -> Result<bool, WasmRuntimeError> {
        runtime
            .log
            .commit_log(CreateInstanceLog {
                agent_name: self.agent_name,
                agent_version: self.agent_version,
                task_id: self.task_id.clone(),
                task_name: self.task_name,
                state: InstanceState::Created,
                fuel_limit: self.policy.compute.as_fuel(),
                fuel_consumed: 0,
            })
            .await?;

        let mut linker = Linker::<WasmState>::new(&runtime.engine);

        add_to_linker_async(&mut linker)?;

        let wasi = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .args(&self.args)
            .envs(&self.policy.env_vars.unwrap_or_default())
            .build();

        let mut limits = StoreLimitsBuilder::new();

        if let Some(ram_bytes) = self.policy.ram {
            limits = limits.memory_size(ram_bytes as usize);
        }

        let limits = limits.build();

        let state = WasmState {
            ctx: wasi,
            table: ResourceTable::new(),
            limits,
        };

        let mut store = Store::new(&runtime.engine, state);

        store.set_fuel(self.policy.compute.as_fuel())?;

        store.limiter(|state| state);

        let component = Component::from_file(&runtime.engine, ".capsule/capsule.wasm")?;

        let instance = linker.instantiate_async(&mut store, &component).await?;

        let run_func = instance.get_typed_func::<(), ()>(&mut store, "wasi:cli/run@0.2.9#run")?;

        run_func.call_async(&mut store, ()).await?;

        Ok(true)
    }
}
