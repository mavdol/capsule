use std::sync::Arc;

use wasmtime::Store;

use crate::config::log::{InstanceState, UpdateInstanceLog};
use crate::wasm::execution_policy::ExecutionPolicy;
use crate::wasm::runtime::{Runtime, RuntimeCommand, WasmRuntimeError};
use crate::wasm::state::{CapsuleAgent, State};

pub struct RunInstance {
    task_id: String,
    policy: ExecutionPolicy,
    store: Store<State>,
    instance: CapsuleAgent,
    args_json: String,
}

impl RunInstance {
    pub fn new(
        task_id: String,
        policy: ExecutionPolicy,
        store: Store<State>,
        instance: CapsuleAgent,
        args_json: String,
    ) -> Self {
        Self {
            task_id,
            policy,
            store,
            instance,
            args_json,
        }
    }
}

impl RuntimeCommand for RunInstance {
    type Output = String;

    async fn execute(mut self, runtime: Arc<Runtime>) -> Result<Self::Output, WasmRuntimeError> {
        runtime
            .log
            .update_log(UpdateInstanceLog {
                task_id: self.task_id.clone(),
                state: InstanceState::Running,
                fuel_consumed: self.policy.compute.as_fuel() - self.store.get_fuel().unwrap_or(0),
            })
            .await?;

        let result = self
            .instance
            .capsule_host_task_runner()
            .call_run(&mut self.store, &self.args_json)
            .await;

        match result {
            Ok(Ok(value)) => {
                runtime
                    .log
                    .update_log(UpdateInstanceLog {
                        task_id: self.task_id,
                        state: InstanceState::Completed,
                        fuel_consumed: self.policy.compute.as_fuel()
                            - self.store.get_fuel().unwrap_or(0),
                    })
                    .await?;
                Ok(value)
            }
            Ok(Err(error_msg)) => {
                runtime
                    .log
                    .update_log(UpdateInstanceLog {
                        task_id: self.task_id,
                        state: InstanceState::Failed,
                        fuel_consumed: self.policy.compute.as_fuel()
                            - self.store.get_fuel().unwrap_or(0),
                    })
                    .await?;
                Ok(format!(r#"{{"error": "{}"}}"#, error_msg))
            }
            Err(e) => {
                runtime
                    .log
                    .update_log(UpdateInstanceLog {
                        task_id: self.task_id,
                        state: InstanceState::Failed,
                        fuel_consumed: self.policy.compute.as_fuel()
                            - self.store.get_fuel().unwrap_or(0),
                    })
                    .await?;

                Err(WasmRuntimeError::WasmtimeError(e))
            }
        }
    }
}
