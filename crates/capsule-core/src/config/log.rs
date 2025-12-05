use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::fmt;

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
    Crashed,
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
    pub gpu_device: Option<u32>,
    pub created_at: i64,
    pub updated_at: i64,
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
            db.create_table("instance_log", &[
                "agent_name TEXT NOT NULL",
                "agent_version TEXT NOT NULL",
                "task_id TEXT NOT NULL",
                "task_name TEXT NOT NULL",
                "state TEXT NOT NULL",
                "fuel_limit INTEGER NOT NULL",
                "fuel_consumed INTEGER NOT NULL",
                "gpu_device INTEGER",
            ], &[])?;

            db.execute("CREATE INDEX IF NOT EXISTS idx_instance_log_task_id ON instance_log(task_id)", [])?;
        }

        Ok(Self {
            db,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_log() {
        let log = Log::new(None, "state.db-wal").unwrap();

        let conn = log.db.conn.lock().unwrap();

        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='instance_log'")
            .expect("Failed to prepare query");

        let mut index_stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name='idx_instance_log_task_id'")
            .expect("Failed to prepare query");

        let exists = stmt.exists([])
            .expect("Failed to check if table exists");

        let index_exists: bool = index_stmt
            .exists([])
            .expect("Failed to check if index exists");

        assert!(exists, "Table instance_log does not exist");
        assert!(index_exists, "Index idx_instance_log_task_id does not exist");
    }
}
