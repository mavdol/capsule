pub fn is_host_allowed(host: &str, allowed_hosts: &Vec<String>) -> bool {

    // also manage subdomains

    // if host is "api.example.com" and allowed_hosts contains "example.com", return true
    // if host is "api.example.com" and allowed_hosts contains "*.example.com", return true
    // if host is "api.example.com" and allowed_hosts contains "api.*", return true
    // if host is "api.example.com" and allowed_hosts contains "api.example.*", return true
    // if host is "api.example.com" and allowed_hosts contains "api.example.com", return true
    // if host is "api.example.com" and allowed_hosts contains "*", return true

    if allowed_hosts.contains(&"*".to_string()) {
        return true;
    }

    if allowed_hosts.contains(&host.to_string()) {
        return true;
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

}
