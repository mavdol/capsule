use serde::Deserialize;

use crate::wasm::execution_policy::{Compute, ExecutionPolicy};

#[derive(Debug, Deserialize, Default)]
pub struct TaskConfig {
    name: Option<String>,
    compute: Option<String>,
    ram: Option<String>,
    timeout: Option<String>,

    #[serde(alias = "maxRetries")]
    max_retries: Option<u64>,
}

impl TaskConfig {
    pub fn to_execution_policy(&self) -> ExecutionPolicy {
        let compute = self
            .compute
            .as_ref()
            .map(|c| match c.to_uppercase().as_str() {
                "LOW" => Compute::Low,
                "MEDIUM" => Compute::Medium,
                "HIGH" => Compute::High,
                _ => c
                    .parse::<u64>()
                    .map(Compute::Custom)
                    .unwrap_or(Compute::Medium),
            });

        let ram = self.ram.as_ref().and_then(|r| Self::parse_ram_string(r));

        ExecutionPolicy::new()
            .name(self.name.clone())
            .compute(compute)
            .ram(ram)
            .timeout(self.timeout.clone())
            .max_retries(self.max_retries)
    }

    pub fn parse_ram_string(s: &str) -> Option<u64> {
        let s = s.trim().to_uppercase();
        if s.ends_with("GB") {
            s.trim_end_matches("GB")
                .trim()
                .parse::<u64>()
                .ok()
                .map(|v| v * 1024 * 1024 * 1024)
        } else if s.ends_with("MB") {
            s.trim_end_matches("MB")
                .trim()
                .parse::<u64>()
                .ok()
                .map(|v| v * 1024 * 1024)
        } else if s.ends_with("KB") {
            s.trim_end_matches("KB")
                .trim()
                .parse::<u64>()
                .ok()
                .map(|v| v * 1024)
        } else {
            s.parse::<u64>().ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ram_string() {
        assert_eq!(
            TaskConfig::parse_ram_string("2GB"),
            Some(2 * 1024 * 1024 * 1024)
        );
        assert_eq!(
            TaskConfig::parse_ram_string("1 GB"),
            Some(1024 * 1024 * 1024)
        );
        assert_eq!(
            TaskConfig::parse_ram_string("4gb"),
            Some(4 * 1024 * 1024 * 1024)
        );

        assert_eq!(
            TaskConfig::parse_ram_string("512MB"),
            Some(512 * 1024 * 1024)
        );
        assert_eq!(
            TaskConfig::parse_ram_string("256 MB"),
            Some(256 * 1024 * 1024)
        );
        assert_eq!(
            TaskConfig::parse_ram_string("128mb"),
            Some(128 * 1024 * 1024)
        );

        assert_eq!(TaskConfig::parse_ram_string("1024KB"), Some(1024 * 1024));
        assert_eq!(TaskConfig::parse_ram_string("512 KB"), Some(512 * 1024));
        assert_eq!(TaskConfig::parse_ram_string("256kb"), Some(256 * 1024));

        assert_eq!(TaskConfig::parse_ram_string("1024"), Some(1024));
        assert_eq!(TaskConfig::parse_ram_string("512"), Some(512));
    }

    #[test]
    fn test_parse_ram_string_invalid() {
        assert_eq!(TaskConfig::parse_ram_string("invalid"), None);
        assert_eq!(TaskConfig::parse_ram_string(""), None);
        assert_eq!(TaskConfig::parse_ram_string("GB"), None);
    }

    #[test]
    fn test_to_execution_policy_default() {
        let config = TaskConfig::default();
        let policy = config.to_execution_policy();

        assert_eq!(policy.name, "default");
        assert_eq!(policy.compute, Compute::Medium);
        assert_eq!(policy.ram, None);
        assert_eq!(policy.timeout, None);
        assert_eq!(policy.max_retries, 0);
    }

    #[test]
    fn test_to_execution_policy_with_values() {
        let config = TaskConfig {
            name: Some("test_task".to_string()),
            compute: Some("HIGH".to_string()),
            ram: Some("2GB".to_string()),
            timeout: Some("30s".to_string()),
            max_retries: Some(3),
        };

        let policy = config.to_execution_policy();

        assert_eq!(policy.name, "test_task");
        assert_eq!(policy.compute, Compute::High);
        assert_eq!(policy.ram, Some(2 * 1024 * 1024 * 1024));
        assert_eq!(policy.timeout, Some("30s".to_string()));
        assert_eq!(policy.max_retries, 3);
    }

    #[test]
    fn test_to_execution_policy_compute_variants() {
        let low = TaskConfig {
            compute: Some("LOW".to_string()),
            ..Default::default()
        };
        assert_eq!(low.to_execution_policy().compute, Compute::Low);

        let medium = TaskConfig {
            compute: Some("MEDIUM".to_string()),
            ..Default::default()
        };
        assert_eq!(medium.to_execution_policy().compute, Compute::Medium);

        let high = TaskConfig {
            compute: Some("HIGH".to_string()),
            ..Default::default()
        };
        assert_eq!(high.to_execution_policy().compute, Compute::High);

        let invalid = TaskConfig {
            compute: Some("INVALID".to_string()),
            ..Default::default()
        };
        assert_eq!(invalid.to_execution_policy().compute, Compute::Medium);
    }
}
