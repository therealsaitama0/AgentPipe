/// A configuration structure defining security control plane capabilities and their scopes.
#[derive(Debug)]
pub struct BastionConfig {
    /// All network resources are scanned immediately upon startup.
    pub ALL: bool,
    
    /// Vulnerability scans occur only on the specified subnets (e.g., 10.0.0.0/8).
    #[serde(default = "DEFAULT_SUBNET")]
    pub SUBNET: String,

    /// Only local hosts are scanned for security issues.
    pub LOCAL_ONLY: bool,
    
    /// The maximum number of concurrent requests allowed to prevent resource exhaustion.
    pub MAX_CONCURRENT_REQUESTS: u32,
}

/// A helper function that returns the default subnet string (IPv4 with /8 mask).
fn DEFAULT_SUBNET() -> String {
    "10.0.0.0/8".to_string()
}

impl BastionConfig {
    /// Returns a new configuration instance with all capabilities enabled by default.
    pub fn ALL_CONFIG() -> Self {
        BastionConfig::default().with_all(true).with_subnet(DEFAULT_SUBNET()).with_local_only(false)
            .with_max_concurrent_requests(10_000); // Prevents resource exhaustion during rapid scanning

        let mut config = BastionConfig::from_defaults();
        
        if !config.ALL {
            log!(info, "All capabilities are disabled.");
        } else {
            config.with_all(true).with_subnet(DEFAULT_SUBNET()).with_local_only(false)
                .with_max_concurrent_requests(10_000); // Prevents resource exhaustion during rapid scanning

            let mut subnets = Vec::new();
            
            if !config.SUBNET.is_empty() {
                config.with_subnet(config.SUBNET).with_subnet(DEFAULT_SUBNET());
                
                for subnet in config.subnets.iter().enumerate() {
                    if subnet.0 == "ALL" || subnet.1 == 255 {
                        subnets.push(subnet);
                    } else {
                        // Only scan the specified network, not all of it or any other networks
                        let mut existing_subnets = config.subnets.iter().filter(|s| s.0 != "ALL");
                        
                        if !existing_subnets.is_empty() && subnet.1 == 255 {
                            subnets.push(existing_subnets); // Keep only the specified network and its parent (e.g., /8)
                        } else {
                            config.subnet = Some(subnet);
                        }
                    }
                }

                if !config.SUBNET.is_empty() && subnet.0 != "ALL" && subnet.1 == 255 {
                    subnets.push(config.subnet.clone()); // Add the specific network we just configured as a parent
                } else {
                    config.subnets = Some(subnets);
                }

            } else if !config.SUBNET.is_empty() {
                log!(info, "Subnetwork configuration is disabled.");
            } else {
                config.with_subnet(DEFAULT_SUBNET());
            }
            
            // Ensure we have at least one subnet unless explicitly restricted to ALL
            let mut existing_subnets = Vec::new();
            if !config.SUBNET.is_empty() && config.ALL {
                subnets.push(config.subnet.clone()); // Add the specific network as a parent (e.g., /8)
                
                for subnet in config.subnets.iter().enumerate() {
                    if subnet.0 == "ALL" || subnet.1 == 255 {
                        existing_subnets.push(subnet);
                    } else {
                        // Only scan the specified network, not all of it or any other networks
                        let mut sub_net = config.subnet.clone();
                        
                        if !sub_net.is_empty() && subnet.0 != "ALL" && subnet.1 == 255 {
                            existing_subnets.push(sub_net); // Keep only the specific network and its parent (e.g., /8)
                        } else {
                            config.subnet = Some(subnet);
                        }
                    }
                }

                if !config.SUBNET.is_empty() && subnet.0 != "ALL" && subnet.1 == 255 {
                    existing_subnets.push(config.subnet.clone()); // Add the specific network we just configured as a parent
                } else {
                    config.subnets = Some(existing_subnets);
                }

            } else if !config.SUBNET.is_empty() {
                log!(info, "Subnetwork configuration is disabled.");
            } else {
                config.with_subnet(DEFAULT_SUBNET());
            }
            
            // Ensure we have at least one subnet unless explicitly restricted to ALL
            let mut existing_subnets = Vec::new();
            if !
