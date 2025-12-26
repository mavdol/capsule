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

        let wasm_future = self
            .instance
            .capsule_host_task_runner()
            .call_run(&mut self.store, &self.args_json);

        let result = match self.policy.timeout_duration() {
            Some(duration) => match tokio::time::timeout(duration, wasm_future).await {
                Ok(inner_result) => inner_result,
                Err(_elapsed) => {
                    runtime
                        .log
                        .update_log(UpdateInstanceLog {
                            task_id: self.task_id.clone(),
                            state: InstanceState::TimedOut,
                            fuel_consumed: self.policy.compute.as_fuel()
                                - self.store.get_fuel().unwrap_or(0),
                        })
                        .await?;

                    runtime
                        .task_reporter
                        .lock()
                        .await
                        .task_failed(&self.policy.name, "Timed out");

                    return Ok(String::new());
                }
            },
            None => wasm_future.await,
        };

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

                runtime
                    .task_reporter
                    .lock()
                    .await
                    .task_failed(&self.policy.name, &error_msg);

                Ok(String::new())
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

                runtime
                    .task_reporter
                    .lock()
                    .await
                    .task_failed(&self.policy.name, &e.to_string());

                Ok(String::new())
            }
        }
    }
}
