use wasmtime::component::Instance;
use wasmtime::Store;

use crate::config::log::{InstanceState, UpdateInstanceLog};
use crate::wasm::execution_policy::ExecutionPolicy;
use crate::wasm::runtime::{Runtime, RuntimeCommand, WasmRuntimeError};
use crate::wasm::state::WasmState;

pub struct StartInstance {
    task_id: String,
    policy: ExecutionPolicy,
    store: Store<WasmState>,
    instance: Instance
}

impl StartInstance {
    pub fn new(task_id: String, policy: ExecutionPolicy, store: Store<WasmState>, instance: Instance) -> Self {
        Self {
            task_id,
            policy,
            store,
            instance
        }
    }
}

impl RuntimeCommand for StartInstance {
    type Output = Store<WasmState>;

    async fn execute(mut self, runtime: &Runtime) -> Result<Self::Output, WasmRuntimeError> {
        runtime
            .log
            .update_log(UpdateInstanceLog {
                task_id: self.task_id.clone(),
                state: InstanceState::Running,
                fuel_consumed: self.policy.compute.as_fuel() - self.store.get_fuel().unwrap_or(0),
            })
            .await?;

        let run_func = self
            .instance
            .get_typed_func::<(), ()>(&mut self.store, "wasi:cli/run@0.2.9#run")?;

        let result = run_func.call_async(&mut self.store, ()).await;


        let output = match result {
            Ok(_) => Ok(self.store),
            Err(e) => {
                runtime
                    .log
                    .update_log(UpdateInstanceLog {
                        task_id: self.task_id,
                        state: InstanceState::Failed,
                        fuel_consumed: self.policy.compute.as_fuel() - self.store.get_fuel().unwrap_or(0),
                    })
                    .await?;

                Err(WasmRuntimeError::WasmtimeError(e)
            )},
        };

        output
    }
}
