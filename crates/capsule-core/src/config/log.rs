use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::fmt;

use crate::storage::database::{Database, DatabaseError};

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
    pub namespace: String,
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
    db: Database,
}

impl Log {
    pub fn new(path: Option<&str>, database_name: &str) -> Result<Self, LogError> {
        let db = Database::new(path, database_name)?;

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
        let log = Log::new(None, "state.db-wal");
        assert!(log.is_ok(), "Failed to create log");
    }
}
