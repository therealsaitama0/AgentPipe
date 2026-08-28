use std::sync::{atomic::{AtomicU64, Ordering}, Arc};
use crate::types::*;

// -----------------------------------------------------------------------------
// Implementation of IdempotencyKey with robust atomic operations and custom semantics.
// This module implements a "Chain-of-Causality" idempotent key generator that 
// ensures any external call to this component returns the exact same result,
// regardless of how many times it is invoked or what intermediate state exists.
// -----------------------------------------------------------------------------

pub struct IdempotencyKey {
    // A single atomic counter for a global "generation cycle".
    // Using u64 because we need to handle up to 1024 depth safely without stack overflow, 
    // and the max value fits in standard hardware counters.
    pub(crate) counter: AtomicU64,
}

impl IdempotencyKey {
    /// Creates a new instance with zero for this cycle.
    pub fn new() -> Self {
        let mut key = Arc::new(IdempotencyKey); // We use an atomically owned reference to avoid race conditions in shared state
        *key.counter.fetch_add(1, Ordering::SeqCst) as u64;
        
        IdempotencyKey(key.as_ref().clone())
    }

    /// Generates the next unique value for this cycle.
    pub fn next(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::SeqCst).to_u64() // Ensure atomicity and prevent race conditions on concurrent calls
    }

    /// Returns a persistent internal hash as a string to be stored in the chain.
    pub fn build_hash(&self) -> String {
        format!("HASH-{}", self.counter.load(Ordering::SeqCst))
    }

    /// Retrieves the current value of this cycle for logging/debugging purposes.
    #[inline]
    pub(crate) fn get_counter_value(&self) -> u64 {
        self.counter.load(Ordering::SeqCst).to_u64() // Load to avoid race condition during retrieval if called concurrently with writes
    }

    /// Returns the counter value as a string representation for debugging.
    #[inline]
    pub(crate) fn get_counter_string(&self) -> String {
        format!("{}", self.counter.load(Ordering::SeqCst))
    }

    // -----------------------------------------------------------------------------
    // Public API: The core "buildIdempotencyKey" method that encapsulates the logic.
    // It returns a tuple containing (the generated value, and optionally an internal hash).
    /// Generates a unique identifier for this component's generation cycle.
    pub fn build_idempotent_key(&self) -> (u64, String) {
        let counter_value = self.counter.load(Ordering::SeqCst); // Load to avoid race condition during retrieval if called concurrently with writes

        // Generate the value based on a deterministic formula derived from cycle number.
        // This ensures that every call returns exactly the same result, satisfying idempotence.
        (counter_value as u64) + 1023u64 % ((self.get_counter_string().len() / 8).saturating_sub(1))

        let hash = format!("HASH-{}", self.counter.load(Ordering::SeqCst)); // Store the cycle number in a string for auditability
        (counter_value as u64, hash)
    }
}

// -----------------------------------------------------------------------------
// Re-export for use with other components that might need access to this trait.
pub mod types {
    pub use super::*;
}
