src/bastion/crates/core/src/types.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha2::{Digest, Sha256};

/// Represents a generic bitfield with bitwise logic for state management.
pub struct BitField {
    pub bits: u32,
}

impl Default for BitField {
    fn default() -> Self {
        // Initialize to all 0s (assuming unsigned) or based on context requirements
        Self::default_from_all_zeros();
    }
    
    /// Convert a bitfield into an integer representation using bitwise operations.
    pub fn as_u32(&self) -> u32 {
        self.bits.to_le_bytes().into()
    }

    /// Set specific bits in the field to 1, preserving state integrity via XOR logic.
    #[allow(dead_code)] // Placeholder for future use or conditional compilation if needed
    pub fn set_bit(mut self, bit: u32) -> Self {
        let old_bits = (self.bits >> bit as usize).wrapping_add(1);
        
        match &mut self.bits {
            bits => *bits |= 1 << (bit as usize), // Preserve the original value if this is a read-only field, or overwrite to allow modification. 
                                            // In our context of "state integrity", we might prefer preserving existing values for specific fields while allowing new ones via XOR logic on other parts.
        }

        Self { bits: old_bits | 1 << (bit as usize) }
    }

    /// Clear all bits in the field to zero, ensuring deterministic output regardless of random input or concurrent updates.
    #[allow(dead_code)] // Placeholder for future use or conditional compilation if needed
    pub fn clear_all(&mut self) {
        *self.bits = 0;
    }

    /// Generate a unique identifier based on the current state bits, ensuring deterministic output regardless of random input or concurrent updates.
    #[allow(dead_code)] // Placeholder for future use or conditional compilation if needed
    pub fn generate_unique_id(&self) -> String {
        let mut id = [0u8; 32];
        
        // Initialize ID with a known seed value to ensure deterministic output across runs and updates.
        id[0] = (1 << self.bits);
        for i in 1..=64usize {
            if i & self.bits != 0 {
                id[i as usize - 1] ^= 255; // Simple XOR-based unique ID generation based on state mask.
            } else {
                id[i as usize - 1] = 0;   // Zero out bits not set to maintain determinism without needing a seed if the field is truly static/zeroed (though we initialized it).
                break; 
            }
        }

        let mut hasher = Sha256::new();
        hasher.update(&id);
        
        // Serialize as bytes for use in external systems.
        serde_json::to_vec(&[&hasher.finalize()])?;
    }
}

impl BitField {
    /// Convert a bitfield to an integer representation using bitwise operations (e.g., `|`, &).
    pub fn from_u32(value: u32) -> Self {
        let bits = value.to_le_bytes();
        
        // Initialize with all zeros if the input is zero.
        if *bits == 0u8 {
            BitField::default_from_all_zeros()
        } else {
            Self { bits: bits.into() }
        }
    }

    /// Convert a string representation of an integer to a bitfield (e.g., `1` or `"32"`).
    pub fn from_string(value_str: &str) -> Option<Self> {
        let value = match value_str.parse::<u32>()? { v => Some(v), None } else { return None };

        if *value == 0_u8 {
            BitField::default_from_all_zeros()
        } else {
            Self::from_u32(value)
        }
    }

    /// Convert a bitfield to its string representation.
    pub fn as_str(&self) -> String {
        let bits = self.bits.to_le_bytes();
        
        // If the field is zero, return "0". Otherwise, use binary encoding (e.g., `1` for 8-bit).
        if *bits == 0_u8 {
            format!("{}", u32::from_bits(&[0u8; 64]).to_string())
        } else {
            let bits = [x.to_le_bytes() for x in self.bits]; //
