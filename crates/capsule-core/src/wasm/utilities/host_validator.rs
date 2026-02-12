pub fn is_host_allowed(host: &str, allowed_hosts: &Vec<String>) -> bool {
    let host_lower = host.to_lowercase();

    for allowed in allowed_hosts {
        let allowed_lower = allowed.to_lowercase();

        if allowed_lower == "*" || allowed_lower == host_lower {
            return true;
        }

        if allowed_lower.contains('*') && matches_wildcard_pattern(&host_lower, &allowed_lower) {
            return true;
        }

        if host_lower != allowed_lower && host_lower.len() > allowed_lower.len() {
            let expected_suffix = format!(".{}", allowed_lower);
            if host_lower.ends_with(&expected_suffix) {
                return true;
            }
        }
    }

    false
}

fn matches_wildcard_pattern(host: &str, pattern: &str) -> bool {
    let host_parts: Vec<&str> = host.split('.').collect();
    let pattern_parts: Vec<&str> = pattern.split('.').collect();

    if pattern_parts.len() > host_parts.len() {
        return false;
    }

    if pattern_parts[0] == "*" && pattern_parts.len() > 1 {
        let suffix_parts = &pattern_parts[1..];
        let host_suffix = &host_parts[host_parts.len() - suffix_parts.len()..];
        return suffix_parts == host_suffix && host_parts.len() == pattern_parts.len();
    }

    if !pattern_parts.is_empty() && pattern_parts[pattern_parts.len() - 1] == "*" {
        let prefix_parts = &pattern_parts[..pattern_parts.len() - 1];
        let host_prefix = &host_parts[..prefix_parts.len()];
        return prefix_parts == host_prefix;
    }

    if pattern_parts.len() == host_parts.len() {
        for (i, pattern_part) in pattern_parts.iter().enumerate() {
            if *pattern_part != "*" && *pattern_part != host_parts[i] {
                return false;
            }
        }
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let allowed = vec!["api.example.com".to_string()];

        assert!(is_host_allowed("api.example.com", &allowed));

        assert!(!is_host_allowed("www.example.com", &allowed));
        assert!(!is_host_allowed("example.com", &allowed));
    }

    #[test]
    fn test_wildcard_all() {
        let allowed = vec!["*".to_string()];

        assert!(is_host_allowed("api.example.com", &allowed));
        assert!(is_host_allowed("www.example.com", &allowed));
        assert!(is_host_allowed("anything.goes.here", &allowed));
    }

    #[test]
    fn test_wildcard_subdomain_prefix() {
        let allowed = vec!["*.example.com".to_string()];

        assert!(is_host_allowed("api.example.com", &allowed));
        assert!(is_host_allowed("www.example.com", &allowed));
        assert!(is_host_allowed("anything.example.com", &allowed));

        assert!(!is_host_allowed("example.com", &allowed));
        assert!(!is_host_allowed("api.other.com", &allowed));
        assert!(!is_host_allowed("deep.api.example.com", &allowed));
    }

    #[test]
    fn test_wildcard_domain_suffix() {
        let allowed = vec!["api.*".to_string()];

        assert!(is_host_allowed("api.example.com", &allowed));
        assert!(is_host_allowed("api.test.org", &allowed));
        assert!(is_host_allowed("api.anything", &allowed));

        assert!(!is_host_allowed("www.example.com", &allowed));
        assert!(!is_host_allowed("api", &allowed));
    }

    #[test]
    fn test_wildcard_tld() {
        let allowed = vec!["api.example.*".to_string()];

        assert!(is_host_allowed("api.example.com", &allowed));
        assert!(is_host_allowed("api.example.org", &allowed));
        assert!(is_host_allowed("api.example.net", &allowed));

        assert!(!is_host_allowed("www.example.com", &allowed));
        assert!(!is_host_allowed("api.other.com", &allowed));
    }

    #[test]
    fn test_parent_domain_matching() {
        let allowed = vec!["example.com".to_string()];
        assert!(is_host_allowed("example.com", &allowed));
        assert!(is_host_allowed("api.example.com", &allowed));
        assert!(is_host_allowed("deep.nested.example.com", &allowed));

        assert!(!is_host_allowed("notexample.com", &allowed));
        assert!(!is_host_allowed("example.org", &allowed));
    }

    #[test]
    fn test_subdomain_only_matching() {
        let allowed = vec!["*.example.com".to_string()];

        assert!(!is_host_allowed("example.com", &allowed));
        assert!(is_host_allowed("api.example.com", &allowed));
        assert!(is_host_allowed("www.example.com", &allowed));

        assert!(!is_host_allowed("deep.api.example.com", &allowed));
    }

    #[test]
    fn test_multiple_allowed_hosts() {
        let allowed = vec![
            "api.example.com".to_string(),
            "*.test.org".to_string(),
            "localhost".to_string(),
        ];

        assert!(is_host_allowed("api.example.com", &allowed));
        assert!(is_host_allowed("www.test.org", &allowed));
        assert!(is_host_allowed("api.test.org", &allowed));
        assert!(is_host_allowed("localhost", &allowed));

        assert!(!is_host_allowed("www.example.com", &allowed));
        assert!(!is_host_allowed("test.org", &allowed));
    }

    #[test]
    fn test_case_insensitive() {
        let allowed = vec!["API.Example.COM".to_string()];

        assert!(is_host_allowed("api.example.com", &allowed));
        assert!(is_host_allowed("API.EXAMPLE.COM", &allowed));
        assert!(is_host_allowed("Api.Example.Com", &allowed));
    }

    #[test]
    fn test_empty_allowed_list() {
        let allowed: Vec<String> = vec![];

        assert!(!is_host_allowed("api.example.com", &allowed));
        assert!(!is_host_allowed("anything", &allowed));
    }

    #[test]
    fn test_complex_wildcard_patterns() {
        let allowed = vec!["api.*.com".to_string()];

        assert!(is_host_allowed("api.example.com", &allowed));

        assert!(!is_host_allowed("www.example.com", &allowed));
    }

    #[test]
    fn test_wildcard_with_ports() {
        let allowed = vec!["*.example.com:8080".to_string()];

        assert!(is_host_allowed("api.example.com:8080", &allowed));

        assert!(!is_host_allowed("api.example.com", &allowed));
    }
}
