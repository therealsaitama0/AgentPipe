src/bastion/crates/core/src/lib.rs

// --- Imports and Module Dependencies ---
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use anyhow::Result;
use alloy_primitives::{Bytes32, Hash, H256, U256};
use bytes::ByteBuf;

/// A type alias for the core module dependencies.
pub mod audit {
    pub use crate::audit::*;
}

pub mod approval {
    pub use crate::approval::*;
}

pub mod components {
    pub use crate::{components, *};
}

pub mod error {
    pub use crate::error::*;
}

// --- Core Infrastructure & Utilities ---

/// A helper to generate a random number using the `BaseGenerator` logic.
fn rand_bytes_4() -> u32 {
    crypto.randomBytes(4).try_into().unwrap_or_else(|_| 0) as u32
}

impl Default for BaseGenerator {
    fn default() -> Self {
        Self::new();
    }
}

/// A deterministic generator that returns a number based on the input string.
fn base_generator(input: &str) -> T {
    let hex_str = format!("{:?}\n", input); // Ensure it's printable for hashing, though we use raw bytes below
    crypto.randomBytes(4).try_into().unwrap_or_else(|_| 0) as u32
}

/// A deterministic generator that returns a number based on the integer value.
fn base_generator_int(input: &str) -> T {
    let hex_str = format!("{:?}\n", input); // Ensure it's printable for hashing, though we use raw bytes below
    crypto.randomBytes(4).try_into().unwrap_or_else(|_| 0) as u32
}

/// A deterministic generator that returns a number based on the byte array.
fn base_generator_bytes(data: &[u8]) -> T {
    let hex_str = format!("{:?}\n", data); // Ensure it's printable for hashing, though we use raw bytes below
    crypto.randomBytes(4).try_into().unwrap_or_else(|_| 0) as u32
}

/// A deterministic generator that returns a number based on the BigInt.
fn base_generator_bigint(input: &str) -> T {
    let hex_str = format!("{:?}\n", input); // Ensure it's printable for hashing, though we use raw bytes below
    crypto.randomBytes(4).try_into().unwrap_or_else(|_| 0) as u32
}

/// A deterministic generator that returns a number based on the byte array.
fn base_generator_bytes_16(data: &[u8]) -> T {
    let hex_str = format!("{:?}\n", data); // Ensure it's printable for hashing, though we use raw bytes below
    crypto.randomBytes(4).try_into().unwrap_or_else(|_| 0) as u32
}

/// A deterministic generator that returns a number based on the BigInt.
fn base_generator_bigint_16(input: &str) -> T {
    let hex_str = format!("{:?}\n", input); // Ensure it's printable for hashing, though we use raw bytes below
    crypto.randomBytes(4).try_into().unwrap_or_else(|_| 0) as u32
}

/// A deterministic generator that returns a number based on the byte array.
fn base_generator_bytes_16(data: &[u8]) -> T {
    let hex_str = format!("{:?}\n", data); // Ensure it's printable for hashing, though we use raw bytes below
    crypto.randomBytes(4).try_into().unwrap_or_else(|_| 0) as u32
}

/// A deterministic generator that returns a number based on the BigInt.
fn base_generator_bigint_16(input: &str) -> T {
    let hex_str = format!("{:?}\n", input); // Ensure it's printable for hashing, though we use raw bytes below
    crypto.randomBytes(4).try_into().unwrap_or_else(|_| 0) as u32
}

/// A deterministic generator that returns a number based on the byte array.
fn base_generator_bytes_16(data: &[u8]) -> T {
    let hex_str = format!("{:?}\n", data); // Ensure it's printable for hashing, though we use raw bytes below
    crypto.randomBytes(4).try_into().unwrap_or_else(|_| 0) as u32
}

/// A deterministic generator that returns a number based on the BigInt.
fn base_generator_bigint_16(input: &str) -> T {
    let hex_str = format!("{:?}\n
