use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Compute {
    Low,
    Medium,
    High,
    Custom(i64),
}

impl fmt::Display for Compute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state_str = match self {
            Compute::Low => 100_000_000,
            Compute::Medium => 2_000_000_000,
            Compute::High => 50_000_000_000,
            Compute::Custom(fuel) => *fuel,
        };
        write!(f, "{}", state_str)
    }
}

#[derive(Clone)]
pub struct ExecutionPolicy {
    name: String,
    compute: Compute,
    ram: Option<i64>,
    timeout: Option<i64>,
    max_retries: i64,
    env_vars: Option<Vec<String>>,
}

impl Default for ExecutionPolicy {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            compute: Compute::Low,
            ram: None,
            timeout: None,
            max_retries: 1,
            env_vars: None,
        }
    }
}

impl ExecutionPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: Option<String>) -> Self {
        if let Some(n) = name {
            self.name = n;
        }
        self
    }

    pub fn compute(mut self, compute: Option<Compute>) -> Self {
        if let Some(c) = compute {
            self.compute = c;
        }
        self
    }

    pub fn ram(mut self, ram: Option<i64>) -> Self {
        self.ram = ram;
        self
    }

    pub fn timeout(mut self, timeout: Option<i64>) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn max_retries(mut self, max_retries: Option<i64>) -> Self {
        if let Some(m) = max_retries {
            self.max_retries = m;
        }
        self
    }

    pub fn env_vars(mut self, env_vars: Option<Vec<String>>) -> Self {
        self.env_vars = env_vars;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_policy() {
        let policy = ExecutionPolicy::new()
            .name(Some("test".to_string()))
            .compute(None)
            .ram(Some(128))
            .timeout(Some(60))
            .max_retries(Some(3))
            .env_vars(None);

        assert_eq!(policy.name, "test");
        assert_eq!(policy.compute, Compute::Low);
        assert_eq!(policy.ram, Some(128));
        assert_eq!(policy.timeout, Some(60));
        assert_eq!(policy.max_retries, 3);
        assert_eq!(policy.env_vars, None);
    }
}
