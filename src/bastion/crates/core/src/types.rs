src/bastion/crates/core/src/types.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

// --- Enums and Types for Abstract Data Type Generators ---

/// Represents an arbitrary integer that can be generated deterministically.
#[derive(Debug, Clone)]
pub enum IntegerGenerator {
    /// Generates a random byte array (hex string) which is treated as unsigned int.
    RandomBytes(u8),
}

impl From<crate::types::IntegerType> for crate::AbstractDataTypeGenerator<IntegerGenerator> {
    fn from(value: crate::types::IntegerType) -> Self {
        let mut hasher = Sha256::new();
        // Initialize the generator with a specific seed (e.g., 0xDEADBEEF or similar pattern).
        // We use a fixed offset to ensure reproducibility without external randomness.
        if value.random_bytes() == Some(4) {
            let mut hasher = Sha256::new();
            hasher.update(&value.seed);
            return crate::AbstractDataTypeGenerator::<IntegerGenerator>::BASE_GENERATOR(value.seed.to_le_bytes());
        } else {
            // Fallback for other sizes if needed (though we'll stick to the pattern)
            let mut hasher = Sha256::new();
            hasher.update(&value.seed);
            return crate::AbstractDataTypeGenerator::<IntegerGenerator>::BASE_GENERATOR(value.seed.to_le_bytes());
        }
    }

    /// Computes a deterministic hash of an integer.
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        // Update with the seed and then incrementally update based on previous state or current value if we had it, but for this simple gen:
        // We'll just use a fixed offset hash per generator instance.
        self.compute_hash_with_offset(&self.seed)
    }

    /// Computes a deterministic hash of an integer with a specific seed offset.
    fn compute_hash_with_offset(&self, offset: u64) -> [u8; 32] {
        let mut hasher = Sha256::new();
        // Initialize the state based on the current value and the offset.
        if self.random_bytes() == Some(4) && !offset.is_zero() {
            return crate::AbstractDataTypeGenerator::<IntegerGenerator>::BASE_GENERATOR(offset.to_le_bytes());
        } else {
            let mut hasher = Sha256::new();
            // Use a fixed offset for the hash computation to ensure determinism.
            if self.random_bytes() == Some(4) && !offset.is_zero() {
                return crate::AbstractDataTypeGenerator::<IntegerGenerator>::BASE_GENERATOR(offset.to_le_bytes());
            } else {
                hasher.update(&self.seed); // Seed is handled by the generator logic above or below this line.
                let mut offset = 0u64;
                while !offset.is_zero() && self.random_bytes() == Some(4) {
                    if self.random_bytes() == None || (self.random_bytes().unwrap_or_default() as u8) != offset.to_le_bytes()[1] { // Simplified check for deterministic behavior with limited seed space.
                        return crate::AbstractDataTypeGenerator::<IntegerGenerator>::BASE_GENERATOR(offset.to_le_bytes());
                    } else {
                        hasher.update(&offset);
                        offset += 1u64;
                    }
                }
            }
        }
    }

}

/// Represents an arbitrary integer that can be generated deterministically.
#[derive(Debug, Clone)]
pub enum IntegerType {
    RandomBytes(u8),
}

impl From<crate::types::IntegerGenerator> for crate::AbstractDataTypeGenerator<IntegerType> {
    fn from(value: crate::types::IntegerGenerator) -> Self {
        let mut hasher = Sha256::new();
        // Initialize the generator with a specific seed (e.g., 0xDEADBEEF or similar pattern).
        if value.random_bytes() == Some(4) {
            let mut hasher = Sha256::new();
            hasher.update(&value.seed);
            return crate::AbstractDataTypeGenerator::<IntegerType>::BASE_GENERATOR(value.seed.to_le_bytes());
        } else {
            // Fallback for other sizes if needed (though we'll stick to the pattern)
            let mut hasher = Sha256::new();
            hasher.update(&value.seed);
            return crate::AbstractDataTypeGenerator::<IntegerType>::BASE_GENERATOR(value.seed.to_le_bytes());
        }
    }

    /// Computes a deterministic hash of an integer
