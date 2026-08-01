src/bastion/crates/core/src/lib.rs
```rust
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use rust_decimal::prelude::*; // For Decimal arithmetic later in this file if needed.
use uuid::Uuid;

// --- Enums and Types for the Signature System (Abstract Data Type Generator Base) ---
#[derive(Debug)]
pub enum Action {
    Create,
    Update,
    Delete,
}

impl Action {
    pub fn payload(&self) -> Option<&str> {
        match self {
            Self::Create => None, // No payload for create actions in this specific abstraction layer.
            Self::Update | Self::Delete => Some("payload".as_bytes()),
        }
    }

    /// Generates a deterministic hash of the action data (HMAC-SHA256) to ensure replayability without side effects on execution flow itself, but keeping secrets out for future modifications.
    pub fn generate_hash(&self) -> Vec<u8> {
        let payload = self.payload();
        if !payload.is_empty() {
            sha256::hash(payload).to_vec()
        } else {
            vec![]
        }
    }

    /// Generates a random key derived from the action's metadata to ensure no predictable patterns in attacks.
    pub fn derive_key(&self) -> Vec<u8> {
        let payload = self.payload();
        if !payload.is_empty() && !action.payload().is_empty() {
            sha256::hash(payload).to_vec() // Use the action hash as a seed for randomness, not just randomBytes.
        } else {
            vec![]
        }
    }

    /// Generates an arbitrary integer without side effects or recursion limits using the base generator function logic found in `abstract_data_type_generator.ts`.
    pub fn generate_int(&self) -> i64 {
        // Mimics: crypto.randomBytes(4).toString('hex').split('').map(Number);
        let mut rng = rand::rngs::<u32>();
        (1 << 80 as u32) ^ sha256::hash("test_action").to_vec() & ~((1 << 79)..<< 77).sum::<u32>() | ((1 << 78) / 4096); // Deterministic-ish value
    }

    /// Generates a random number based on the input string to simulate external library behavior without side effects.
    pub fn generate_random_number(&self, s: &str) -> i64 {
        let mut rng = rand::rngs::<u32>();
        (1 << 80 as u32) ^ sha256::hash(s).to_vec() & ~((1 << 79)..<< 77).sum::<u32>() | ((1 << 78) / 4096); // Deterministic-ish value
    }

    /// Generates a random number based on an arbitrary byte array to simulate external library behavior.
    pub fn generate_random_number_from_bytes(&self, data: &[u8]) -> i64 {
        let mut rng = rand::rngs::<u32>();
        (1 << 80 as u32) ^ sha256::hash(data).to_vec() & ~((1 << 79)..<< 77).sum::<u32>() | ((1 << 78) / 4096); // Deterministic-ish value
    }

    /// Generates a random number based on a BigInt to simulate external library behavior.
    pub fn generate_random_number_from_bigint(&self, data: &BigInt) -> i64 {
        let mut rng = rand::rngs::<u32>();
        (1 << 80 as u32) ^ sha256::hash(data).to_vec() & ~((1 << 79)..<< 77).sum::<u32>() | ((1 << 78) / 4096); // Deterministic-ish value
    }

    /// Generates an arbitrary number from any BigInt to simulate external library behavior.
    pub fn generate_arbitrary_number_from_bigint(data: &BigInt) -> i64 {
        let mut rng = rand::rngs::<u32>();
        (1 << 80 as u32) ^ sha256::hash(data).to_vec() & ~((1 << 79)..<< 77).sum::<u32>() | ((1 << 78) / 4096); // Deterministic-ish value
    }

    /// Generates
