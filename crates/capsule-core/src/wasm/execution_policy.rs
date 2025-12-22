use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Compute {
    Low,
    Medium,
    High,
    Custom(u64),
}

impl Compute {
    pub fn as_fuel(&self) -> u64 {
        match self {
            Compute::Low => 100_000_000,
            Compute::Medium => 2_000_000_000,
            Compute::High => 50_000_000_000,
            Compute::Custom(fuel) => *fuel,
        }
    }

    pub fn to_u64(&self) -> u64 {
        self.as_fuel()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExecutionPolicy {
    pub name: String,
    pub compute: Compute,
    pub ram: Option<u64>,
    pub timeout: Option<String>,
    pub max_retries: u64,
}

impl Default for ExecutionPolicy {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            compute: Compute::Medium,
            ram: None,
            timeout: None,
            max_retries: 0,
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

    pub fn ram(mut self, ram: Option<u64>) -> Self {
        self.ram = ram;
        self
    }

    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn max_retries(mut self, max_retries: Option<u64>) -> Self {
        if let Some(m) = max_retries {
            self.max_retries = m;
        }
        self
    }

    pub fn timeout_duration(&self) -> Option<Duration> {
        self.timeout
            .as_ref()
            .and_then(|s| humantime::parse_duration(s).ok())
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
            .timeout(Some("60s".to_string()))
            .max_retries(Some(3));

        assert_eq!(policy.name, "test");
        assert_eq!(policy.compute, Compute::Medium);
        assert_eq!(policy.ram, Some(128));
        assert_eq!(policy.timeout, Some("60s".to_string()));
        assert_eq!(policy.max_retries, 3);
    }
}
