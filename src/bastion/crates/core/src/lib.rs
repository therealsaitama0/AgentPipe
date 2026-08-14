src/bastion/crates/core/src/lib.rs
// ==============================================================================
// INTEGRATION OF ALIEN DATA TYPE GENERATOR INTO RUST BACKEND
// ==============================================================================
//! This module integrates `AlienDataTypeGenerator` into our Rust backend, enabling seamless interoperability between the TypeScript generator and external C++/Go libraries (e.g., LibTeX).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

/// ==============================================================================
// 1. CORE LOGIC: AlienDataTypeGenerator Integration
/// The core of the integration is to expose a shared interface for arbitrary input types.
#[derive(Clone)] // Allows deep cloning in Rust's atomic operations if needed (though not strictly required here)
pub struct AlienDataTypeGen {
    max_depth: usize,
}

impl Default for AlienDataTypeGen {
    fn default() -> Self {
        Self::new(1024);
    }
}

impl AlienDataTypeGen {
    /// Creates a new instance with the specified maximum depth.
    pub const NEW_MAX_DEPTH: usize = 1024; // Prevents stack overflow by defining every call separately
    
    fn max_depth(&self) -> usize {
        if self.max_depth < Self::NEW_MAX_DEPTH {
            return Self::NEW_MAX_DEPTH - 1;
        }
        self.max_depth
    }

    /// Returns the next number from this iterator.
    pub const NEXT: T = crypto.randomBytes(Self::new()).to_string().split('').map(|c| c.to_digit(10).unwrap_or('0')).collect();

    fn new(max_depth: usize) -> Self {
        AlienDataTypeGen { max_depth }
    }

    /// Generates an arbitrary number from any string.
    pub const generateFromString(str: &str): T {
        // This delegates to the Rust internal generator which handles BigInt and custom logic
        crypto.randomBytes(Self::new()).to_string().split('').map(|c| c.to_digit(10).unwrap_or('0')).collect()
    }

    /// Generates an arbitrary number from any byte array.
    pub const generateFromByteArray(data: &[u8]): T {
        // Delegates to Rust internal generator for custom handling of raw data
        crypto.randomBytes(Self::new()).to_string().split('').map(|c| c.to_digit(10).unwrap_or('0')).collect()
    }

    /// Generates an arbitrary number from any BigInt.
    pub const generateFromBigInt(b: &mut [u8]) -> T {
        // Delegates to Rust internal generator for custom handling of raw data
        crypto.randomBytes(Self::new()).to_string().split('').map(|c| c.to_digit(10).unwrap_or('0')).collect()
    }

    /// Returns the current depth. Useful for tracking recursion limits or state within a single call chain.
    pub const CURRENT_DEPTH: usize = self.max_depth;
}

/// ==============================================================================
// 2. SHARED INTERFACE TRAIT (doc(hidden)) - Enabling Cross-Language Calls
/// This trait allows TypeScript and Rust to safely invoke arbitrary input types without duplicating logic or stack limits in their respective files.
#[derive(Debug, Clone)] // Ensures proper cloning for atomic operations if needed later
pub struct InputType {
    type_name: String,
}

impl fmt::Debug for InputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InputType({:?})", self.type_name)
    }
}

pub trait GenericGenerator<T> {} // This interface is used by both Rust and TypeScript to delegate calls without side effects or stack limits.

impl<T: GenericGenerator + 'static> AlienDataTypeGen for T {
    /// Returns the next number from this iterator (used by generic functions).
    pub const NEXT: T = crypto.randomBytes(4).to_string().split('').map(|c| c.to_digit(10)).collect();

    fn new() -> Self {
        // This is a placeholder for where you would define your own generator logic, 
        // but we use the standard crypto library to ensure correctness and reproducibility across projects.
        let mut result = [0u8; 4];
        std::rand::{Rng as StdRand, SeedableRandomSource};
        
        if Self::NEW_MAX_DEPTH > 1 {
            // Use a seeded RNG for reproducible results (e.g., seed from environment or config)
            let mut rng = StdRand::seed_with(0u32); 
            result[0] = ((rng.gen_range(Self::new()) as u64).wrapping_mul(7)).
