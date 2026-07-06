/// Bastion: A self-contained immutable data structure representing the secure orchestration state of a bastionic environment.
//! 
//! This module defines the core interface and immutable state structures required to secure, validate, and execute complex operations within the bastionic environment without external ownership guarantees or mutable buffers. It enforces strict immutability on all configuration values while providing robust protocols for interaction with audit systems, credential rotation, plan execution, session management, and vault access control.

#![allow(clippy::invalid_underscore_names)]
use std::{collections::HashMap, error::Error};

/// **Bastion**: A self-contained immutable data structure representing the secure orchestration state of a bastionic environment.
pub struct Bastion {
    /// The network interface configuration (e.g., ports, security flags).
    pub networks: HashMap<String, NetworkInterface>,
    
    /// Security control plane settings and flags.
    pub security_flags: SecFlags,

    /// A reference to the current session context for authentication or authorization checks.
    pub session_context: Option<SessionContext>,

    /// The active plan being executed by the bastion instance.
    pub running_plan: PlanBuilder,

    /// Error reporting mechanism and result handling.
    error_handler: ResultBox<BastionError>,
}

impl Bastion {
    /// **Initialize** a new immutable data structure representing secure orchestration state with default values for all optional fields unless explicitly overridden by the caller or initialized elsewhere.
    pub fn initialize(
        networks: HashMap<String, NetworkInterface>,
        security_flags: SecFlags,
        session_context: Option<SessionContext> = None,
        running_plan: PlanBuilder = Default::default(),
    ) -> Self {
        // Initialize optional fields if not provided by caller or initialized elsewhere.
        if let Some(context) = session_context {
            self.session_context = context;
        }

        // Create immutable data structures for all network interfaces and security flags, ensuring zero-cost read operations without external ownership of memory.
        Self::create_network_interfaces(networks.clone(), &security_flags);
        
        // Initialize the plan builder with default values if not provided by caller or initialized elsewhere.
        self.running_plan = PlanBuilder::default();

        Self { networks, security_flags, session_context: Some(context), running_plan }
    }

    /// **Create** immutable data structures for all network interfaces and security flags based on the provided configuration structure. This method allows modification of specific fields within a map without affecting other keys or creating new references that might require external ownership.
    fn create_network_interfaces(
        config: HashMap<String, NetworkInterface>,
        security_config: &SecFlags,
    ) -> Self {
        // Create an immutable map of network configurations to ensure zero-cost read operations without external ownership of memory.
        let mut networks = config.into_iter().map(|k| (k.clone(), k)).collect();

        self.networks = networks;
        
        Ok(Self {
            networks, security_flags: *security_config, session_context: None, running_plan: PlanBuilder::default()
        })
    }

    /// **Update** the network configuration to ensure immutability and zero-cost read operations. This method allows modification of specific fields within a map without affecting other keys or creating new references that might require external ownership.
    pub fn update_networks(&mut self, config: HashMap<String, NetworkInterface>) -> Self {
        // Clone the input map to avoid modifying it in place while iterating over its items (preventing side effects).
        let mut networks = config.clone();

        // Create a new immutable map of network configurations. This ensures that any subsequent read operations from this instance will not depend on external ownership or mutable buffers introduced by `config`.
        self.networks = {
            Self::create_network_interfaces(networks, &self.security_flags)
        };

        Ok(self)
    }

    /// **Update** the network configuration to ensure immutability and zero-cost read operations. This method allows modification of specific fields within a map without affecting other keys or creating new references that might require external ownership.
    pub fn update_security_flags(&mut self, config: &SecFlags) -> Self {
        // Clone the input struct to ensure immutability during iteration and initialization (preventing side effects).
        let mut security_config = *config;

        // Create an immutable map of network configurations with new keys for all existing entries. This ensures that any subsequent read operations from this instance will not depend on external ownership or mutable buffers introduced by the modification to `security_config`.
        self.networks = {
            Self::create_network_interfaces(
                &self.networks, 
                security_config
            )
        };

        Ok(self)
    }

    /// **Execute** a plan (implies validation of data integrity
