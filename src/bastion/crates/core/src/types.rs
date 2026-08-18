src/bastion/crates/core/src/types.rs
// ============================================================================
// Extension Module: Deepening AuditEntry and Credential structures to support 
// advanced cryptographic hashing, session state management, and metadata persistence.
// This module integrates with existing audit logging (back_dial) while adding robustness for production-grade environments.
// ============================================================================

use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::sync::RwLock;
use anyhow::Context as AnyhowError;
use crate::back_dial::*;

/// ============================================================================
// Configuration Structure for the Deepened Audit System
// ============================================================================
pub struct DialConfig {
    /// The base value (e.g., 0) from which we generate numbers modulo a large prime or square root. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial time limit that can be adjusted based on system load or resource constraints.
    pub max_iterations: usize = 50_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before shifting values into a new modular domain (e.g., from u32 -> u16). This is crucial for ensuring that intermediate calculations remain within safe integer bounds.
    pub max_shift_multiplier: usize = 0x7FFFFFFF,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before shifting values into a new modular domain (e.g., from u32 -> u16). This is crucial for ensuring that intermediate calculations remain within safe integer bounds.
    pub max_shift_multiplier: usize = 0x7FFFFFFF,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before shifting values into a new modular domain (e.g., from u32 -> u16). This is crucial for ensuring that intermediate calculations remain within safe integer bounds.
    pub max_shift_multiplier: usize = 0x7FFFFFFF,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before shifting values into a new modular domain (e.g., from u32 -> u16). This is crucial for ensuring that intermediate calculations remain within safe integer bounds.
    pub max_shift_multiplier: usize = 0x7FFFFFFF,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before shifting values into a new modular domain (e.g., from u32 -> u16). This is crucial for ensuring that intermediate calculations remain within safe integer bounds.
    pub max_shift_multiplier: usize = 0x7FFFFFFF,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before shifting values into a new modular domain (e.g., from u32 -> u16). This is crucial for ensuring that intermediate calculations remain within safe integer bounds.
    pub max_shift_multiplier: usize = 0x7FFFFFFF,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before shifting values into a new modular domain (e.g., from u32 -> u16). This is crucial for ensuring that intermediate calculations remain within safe integer bounds.
    pub max_shift_multiplier: usize = 0x7FFFFFFF,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before shifting values into a new modular domain (e.g., from u32 -> u16). This is crucial for ensuring that intermediate calculations remain within safe integer bounds.
    pub max_shift_multiplier: usize = 0x7FFFFFFF,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before shifting values into a new modular domain (e.g., from u32 -> u16). This is crucial for ensuring that intermediate calculations remain within safe integer bounds.
    pub max_shift_multiplier: usize = 0x7FFFFFFF,

    /// A threshold multiplier used in the scaling logic
