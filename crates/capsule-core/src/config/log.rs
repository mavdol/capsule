use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::config::database::{Database, DatabaseError};

#[derive(Debug)]
pub enum LogError {
    DatabaseError(String),
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogError::DatabaseError(msg) => write!(f, "Log error > {}", msg),
        }
    }
}

impl From<DatabaseError> for LogError {
    fn from(err: DatabaseError) -> Self {
        LogError::DatabaseError(err.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstanceState {
    Created,
    Running,
    Completed,
    Failed,
    Interrupted,
}

impl fmt::Display for InstanceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state_str = match self {
            InstanceState::Created => "created",
            InstanceState::Running => "running",
            InstanceState::Completed => "completed",
            InstanceState::Failed => "failed",
            InstanceState::Interrupted => "interrupted",
        };
        write!(f, "{}", state_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceLog {
    pub id: Uuid,
    pub agent_name: String,
    pub agent_version: String,
    pub task_id: String,
    pub task_name: String,
    pub state: InstanceState,
    pub fuel_limit: u64,
    pub fuel_consumed: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstanceLog {
    pub agent_name: String,
    pub agent_version: String,
    pub task_id: String,
    pub task_name: String,
    pub state: InstanceState,
    pub fuel_limit: u64,
    pub fuel_consumed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInstanceLog {
    pub task_id: String,
    pub state: InstanceState,
    pub fuel_consumed: u64,
}

#[derive(Clone)]
pub struct Log {
    pub db: Database,
}

impl Log {
    pub fn new(path: Option<&str>, database_name: &str) -> Result<Self, LogError> {
        let db = Database::new(path, database_name)?;

        let table_exists = db.table_exists("instance_log")?;

        if !table_exists {
            db.create_table(
                "instance_log",
                &[
                    "agent_name TEXT NOT NULL",
                    "agent_version TEXT NOT NULL",
                    "task_id TEXT NOT NULL",
                    "task_name TEXT NOT NULL",
                    "state TEXT NOT NULL",
                    "fuel_limit INTEGER NOT NULL",
                    "fuel_consumed INTEGER NOT NULL",
                ],
                &[],
            )?;

            db.execute(
                "CREATE INDEX IF NOT EXISTS idx_instance_log_task_id ON instance_log(task_id)",
                [],
            )?;

            db.execute(
                "CREATE INDEX IF NOT EXISTS idx_instance_log_created_at ON instance_log(created_at)",
                [],
            )?;
        }

        Ok(Self { db })
    }

    pub fn commit_log<F>(&self, log: CreateInstanceLog, on_commit: F) -> Result<(), LogError>
    where
        F: FnOnce(Result<(), LogError>) + Send + 'static,
    {
        match self.db.execute(
            "INSERT INTO instance_log (id, agent_name, agent_version, task_id, task_name, state, fuel_limit, fuel_consumed) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            [
                &Uuid::new_v4().to_string(),
                &log.agent_name,
                &log.agent_version,
                &log.task_id,
                &log.task_name,
                &log.state.to_string(),
                &log.fuel_limit.to_string(),
                &log.fuel_consumed.to_string()
            ],
        ) {
            Ok(_) => on_commit(Ok(())),
            Err(e) => on_commit(Err(LogError::DatabaseError(e.to_string()))),
        }

        Ok(())
    }

    pub fn update_log(&self, log: UpdateInstanceLog) -> Result<(), LogError> {
        self.db.execute(
            "UPDATE instance_log SET state = ?, fuel_consumed = ? WHERE task_id = ?",
            [
                &log.state.to_string(),
                &log.fuel_consumed.to_string(),
                &log.task_id.to_string(),
            ],
        )?;

        Ok(())
    }

    pub fn get_logs(&self, task_id: &str) -> Result<Vec<InstanceLog>, LogError> {
        let logs = self.db.query(
            "SELECT id, agent_name, agent_version, task_id, task_name, state, fuel_limit, fuel_consumed, created_at, updated_at FROM instance_log WHERE task_id = ? ORDER BY created_at DESC",
            [task_id],
            |row| {
                let id_str: String = row.get(0)?;
                let state_str: String = row.get(5)?;
                Ok(InstanceLog {
                    id: Uuid::parse_str(&id_str).map_err(|_| DatabaseError::InvalidQuery("Invalid UUID".to_string()))?,
                    agent_name: row.get(1)?,
                    agent_version: row.get(2)?,
                    task_id: row.get(3)?,
                    task_name: row.get(4)?,
                    state: match state_str.as_str() {
                        "created" => InstanceState::Created,
                        "running" => InstanceState::Running,
                        "completed" => InstanceState::Completed,
                        "failed" => InstanceState::Failed,
                        "interrupted" => InstanceState::Interrupted,
                        _ => return Err(DatabaseError::InvalidQuery(format!("Invalid state: {}", state_str))),
                    },
                    fuel_limit: row.get::<_, i64>(6)? as u64,
                    fuel_consumed: row.get::<_, i64>(7)? as u64,
                    created_at: row.get(8)?,
                    updated_at: row.get(9)?,
                })
            }
        )?;

        Ok(logs)
    }

    pub fn delete_log(&self, task_id: &str) -> Result<(), LogError> {
        self.db
            .execute("DELETE FROM instance_log WHERE task_id = ?", [task_id])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod creation {
        use super::*;

        #[test]
        fn test_new_log() {
            let log = Log::new(None, "state.db-wal").unwrap();

            let conn = log.db.conn.lock().unwrap();

            let mut stmt = conn
                .prepare(
                    "SELECT name FROM sqlite_master WHERE type='table' AND name='instance_log'",
                )
                .expect("Failed to prepare query");

            let mut index_stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name='idx_instance_log_task_id'")
                .expect("Failed to prepare query");

            let exists = stmt.exists([]).expect("Failed to check if table exists");

            let index_exists: bool = index_stmt
                .exists([])
                .expect("Failed to check if index exists");

            assert!(exists, "Table instance_log does not exist");
            assert!(
                index_exists,
                "Index idx_instance_log_task_id does not exist"
            );
        }

        #[test]
        fn test_commit_log() {
            use std::sync::Arc;
            use std::sync::Mutex;

            let log = Log::new(None, "state.db-wal").unwrap();

            let callback_invoked = Arc::new(Mutex::new(false));
            let callback_invoked_clone = callback_invoked.clone();

            let _ = log.commit_log(
                CreateInstanceLog {
                    agent_name: "agent_name".to_string(),
                    agent_version: "agent_version".to_string(),
                    task_id: "task_id".to_string(),
                    task_name: "task_name".to_string(),
                    state: InstanceState::Created,
                    fuel_limit: 100,
                    fuel_consumed: 0,
                },
                move |result| {
                    *callback_invoked_clone.lock().unwrap() = true;
                    assert!(result.is_ok(), "Commit should succeed: {:?}", result);
                },
            );

            assert!(
                *callback_invoked.lock().unwrap(),
                "Callback should have been invoked"
            );

            let conn = log.db.conn.lock().unwrap();

            let mut stmt = conn
                .prepare("SELECT task_name FROM instance_log WHERE task_id = 'task_id'")
                .expect("Failed to prepare query");

            let exists = stmt.exists([]).expect("Failed to check if instance exists");

            assert!(exists, "instance does not exist");
        }

        #[test]
        fn test_update_log() {
            let log = Log::new(None, "state.db-wal").unwrap();

            {
                let conn = log.db.conn.lock().unwrap();

                conn.execute("INSERT INTO instance_log (id, agent_name, agent_version, task_id, task_name, state, fuel_limit, fuel_consumed) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", [
                    &Uuid::new_v4().to_string(),
                    &"test_agent".to_string(),
                    &"1.0.0".to_string(),
                    &"test_task_123".to_string(),
                    &"Test Task".to_string(),
                    &"created".to_string(),
                    "15000000",
                    "0",
                ]).expect("Failed to insert test data");
            }

            let _ = log.update_log(UpdateInstanceLog {
                task_id: "test_task_123".to_string(),
                state: InstanceState::Running,
                fuel_consumed: 10,
            });

            let conn = log.db.conn.lock().unwrap();

            let state: String = conn
                .query_row(
                    "SELECT state FROM instance_log WHERE task_id = 'test_task_123'",
                    [],
                    |row| row.get(0),
                )
                .expect("Failed to query state");

            let fuel_consumed: i64 = conn
                .query_row(
                    "SELECT fuel_consumed FROM instance_log WHERE task_id = 'test_task_123'",
                    [],
                    |row| row.get(0),
                )
                .expect("Failed to query fuel_consumed");

            assert_eq!(state, "running", "State should be updated to running");
            assert_eq!(fuel_consumed, 10, "Fuel consumed should be updated to 10");
        }
    }

    mod queries {
        use super::*;

        #[test]
        fn test_get_logs() {
            let log = Log::new(None, "state.db-wal").unwrap();

            {
                let conn = log.db.conn.lock().unwrap();

                conn.execute("INSERT INTO instance_log (id, agent_name, agent_version, task_id, task_name, state, fuel_limit, fuel_consumed, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
                    &Uuid::new_v4().to_string(),
                    "test_agent",
                    "1.0.0",
                    "test_task_123",
                    "Test Task",
                    "created",
                    "10000",
                    "0",
                    "1000",
                    "1000",
                ]).expect("Failed to insert first test log");

                conn.execute("INSERT INTO instance_log (id, agent_name, agent_version, task_id, task_name, state, fuel_limit, fuel_consumed, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
                    &Uuid::new_v4().to_string(),
                    "test_agent",
                    "1.0.0",
                    "test_task_123",
                    "Test Task",
                    "running",
                    "10000",
                    "5000",
                    "2000",
                    "2000",
                ]).expect("Failed to insert second test log");

                conn.execute("INSERT INTO instance_log (id, agent_name, agent_version, task_id, task_name, state, fuel_limit, fuel_consumed, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
                    &Uuid::new_v4().to_string(),
                    "test_agent",
                    "1.0.0",
                    "test_task_123",
                    "Test Task",
                    "completed",
                    "10000",
                    "8500",
                    "3000",
                    "3000",
                ]).expect("Failed to insert third test log");

                conn.execute("INSERT INTO instance_log (id, agent_name, agent_version, task_id, task_name, state, fuel_limit, fuel_consumed, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
                    &Uuid::new_v4().to_string(),
                    "other_agent",
                    "2.0.0",
                    "other_task_456",
                    "Other Task",
                    "failed",
                    "5000",
                    "2500",
                    "1500",
                    "1500",
                ]).expect("Failed to insert other task log");
            }

            let logs = log.get_logs("test_task_123").expect("Failed to get logs");

            assert_eq!(logs.len(), 3, "Expected 3 logs for test_task_123");

            assert_eq!(
                logs[0].state.to_string(),
                "completed",
                "First log should be completed"
            );
            assert_eq!(
                logs[0].fuel_consumed, 8500,
                "First log fuel_consumed should be 8500"
            );
            assert_eq!(
                logs[0].created_at, 3000,
                "First log created_at should be 3000"
            );

            assert_eq!(
                logs[1].state.to_string(),
                "running",
                "Second log should be running"
            );
            assert_eq!(
                logs[1].fuel_consumed, 5000,
                "Second log fuel_consumed should be 5000"
            );
            assert_eq!(
                logs[1].created_at, 2000,
                "Second log created_at should be 2000"
            );

            assert_eq!(
                logs[2].state.to_string(),
                "created",
                "Third log should be created"
            );
            assert_eq!(
                logs[2].fuel_consumed, 0,
                "Third log fuel_consumed should be 0"
            );
            assert_eq!(
                logs[2].created_at, 1000,
                "Third log created_at should be 1000"
            );

            for log_entry in &logs {
                assert_eq!(
                    log_entry.task_id, "test_task_123",
                    "All logs should have task_id test_task_123"
                );
                assert_eq!(
                    log_entry.agent_name, "test_agent",
                    "All logs should have agent_name test_agent"
                );
                assert_eq!(
                    log_entry.task_name, "Test Task",
                    "All logs should have task_name Test Task"
                );
            }

            let empty_logs = log
                .get_logs("non_existent_task")
                .expect("Failed to get logs for non-existent task");
            assert_eq!(empty_logs.len(), 0, "Expected 0 logs for non-existent task");
        }

        #[test]
        fn test_delete_log() {
            let log = Log::new(None, "state.db-wal").unwrap();

            {
                let conn = log.db.conn.lock().unwrap();

                conn.execute("INSERT INTO instance_log (id, agent_name, agent_version, task_id, task_name, state, fuel_limit, fuel_consumed, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
                    &Uuid::new_v4().to_string(),
                    "test_agent",
                    "1.0.0",
                    "task_to_delete",
                    "Task To Delete",
                    "created",
                    "10000",
                    "0",
                    "1000",
                    "1000",
                ]).expect("Failed to insert first test log");

                conn.execute("INSERT INTO instance_log (id, agent_name, agent_version, task_id, task_name, state, fuel_limit, fuel_consumed, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
                    &Uuid::new_v4().to_string(),
                    "test_agent",
                    "1.0.0",
                    "task_to_delete",
                    "Task To Delete",
                    "running",
                    "10000",
                    "5000",
                    "2000",
                    "2000",
                ]).expect("Failed to insert second test log");

                conn.execute("INSERT INTO instance_log (id, agent_name, agent_version, task_id, task_name, state, fuel_limit, fuel_consumed, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
                    &Uuid::new_v4().to_string(),
                    "other_agent",
                    "2.0.0",
                    "task_to_keep",
                    "Task To Keep",
                    "completed",
                    "5000",
                    "2500",
                    "1500",
                    "1500",
                ]).expect("Failed to insert task to keep");
            }

            let logs_before = log
                .get_logs("task_to_delete")
                .expect("Failed to get logs before deletion");
            assert_eq!(
                logs_before.len(),
                2,
                "Expected 2 logs for task_to_delete before deletion"
            );

            log.delete_log("task_to_delete")
                .expect("Failed to delete logs");

            let logs_after = log
                .get_logs("task_to_delete")
                .expect("Failed to get logs after deletion");
            assert_eq!(
                logs_after.len(),
                0,
                "Expected 0 logs for task_to_delete after deletion"
            );

            let kept_logs = log
                .get_logs("task_to_keep")
                .expect("Failed to get logs for task_to_keep");
            assert_eq!(
                kept_logs.len(),
                1,
                "Expected 1 log for task_to_keep to remain"
            );
            assert_eq!(
                kept_logs[0].task_id, "task_to_keep",
                "Kept log should have correct task_id"
            );
            assert_eq!(
                kept_logs[0].state.to_string(),
                "completed",
                "Kept log should have correct state"
            );
        }
    }
}
