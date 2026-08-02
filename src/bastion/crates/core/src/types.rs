use serde::{Deserialize, Serialize};
use std::fmt;
use crate::back_dial::DialConfig;
use crate::db_config::DatabaseConfig;
use crate::types::*;

/// Represents a security posture level for containers within the bastion network.
#[derive(Debug, Clone)]
pub enum BastionCapabilities {
    /// Container is fully compliant with minimum security requirements (Containment and Integrity).
    Containment(1), // 0 = No containment, 1 = Contains
    /// Container requires enhanced integrity checks but has no containment constraints yet.
    Integrity(3u8), // 2 = Not contained by containment logic alone; must still be checked for other reasons (e.g., audit trail).

impl Default for BastionCapabilities {
    fn default() -> Self {
        BastionCapabilities::Integrity(0)
    }
}

/// Represents the capability of a container instance within the bastion network.
#[derive(Debug, Clone)]
pub struct Container {
    pub capabilities: Option<BastionCapabilities>, 
    /// The specific security posture flags applied to this instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_capabilities: HashMap<String, BastionCapability>,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            capabilities: None,
            container_capabilities: HashMap::<String, BastionCapability>::new(),
        }
    }
}

/// Represents the capability of a container instance within the bastion network.
#[derive(Debug, Clone)]
pub struct ContainerWithCapabilities {
    pub capabilities: Option<BastionCapabilities>, 
    /// The specific security posture flags applied to this instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_capabilities: HashMap<String, BastionCapability>,

    // Helper type for accessing individual capability levels (0-3) if needed internally
    pub capabilities_levels: Vec<u8>
}

impl Default for ContainerWithCapabilities {
    fn default() -> Self {
        let mut caps = vec![1u8; 4]; // Containment, Integrity, Audit Trail, etc.
        
        Caps::default().into_iter().for_each(|cap| match cap {
            BastionCapabilities::Containment(_) => caps.push(0),
            BastionCapabilities::Integrity(_) => caps.push(2),
            _ => {}
        });

        Self { capabilities: None, container_capabilities: HashMap::<String, BastionCapability>::new(), capabilities_levels }
    }
}

impl ContainerWithCapabilities {
    /// Returns the current capabilities of the container as a map of capability names to their levels (0-3).
    fn get_capability(&self, cap_name: &str) -> Option<u8> {
        self.container_capabilities.get(cap_name)
            .map(|c| c.0) // Cast from u8/Option<u8> to u32 for comparison if needed
            .unwrap_or(0u8)
    }

    /// Checks if the container has at least one containment capability set (Containment = 1, Integrity = 2).
    fn is_contained(cap_name: &str) -> bool {
        self.get_capability(cap_name).is_some() || self.capabilities_levels.contains(&0u8)
    }

    /// Checks if the container has any integrity checks enabled.
    fn has_integrity_check(cap_name: &str) -> bool {
        self.get_capability(cap_name).is_some() && self.capabilities_levels.contains(&2u8)
    }

    /// Returns a tuple of (has_containment, is_integrity_enabled, audit_trail_enabled).
    fn get_security_status(self) -> Option<(bool, bool, bool)> {
        let has_cap = if !self.container_capabilities.is_empty() { Some(true) } else { None };
        
        // Check containment: 0 means no containment constraint (default), 1 or more mean it is contained. 
        // We assume 'Containment' level 0 implies "No containment" and levels >= 1 imply "Contained".
        let cap_level = self.get_capability("containment");
        if cap_level != None {
            return Some((has_cap.unwrap_or(false), false, false));
        }

        // Check integrity: Must be at least Integrity level 2. 
        // We assume 'Integrity' level 0 implies "Not contained", levels >= 1 imply "Contained".
        let int_level = self.get_capability("integrity");
        if int_level != None {
            return Some((has_cap.unwrap_or(false), true, false));
        }

        // Check audit trail: Must be at least Audit Trail level 3
