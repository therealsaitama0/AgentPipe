// src/bastion/crates/core/src/lib.rs

use std::io::{self, Write};
use crate::audit::AuditChain;
use crate::approval::ApprovalTicket;
use crate::components::*;
use crate::error::BastionError;
use crate::firecracker::{FirecrackerAdapter, FirecrackerConfig, VmInstance, VmState};
use crate::forced_command::ForcedCommandConfig;
use crate::network_guard::NetworkGuard;
use crate::policy::{PolicyDecision, PolicyEngine};
use crate::script_executor::ScriptExecutor;
use crate::session::SessionManager;
use crate::types::*;

/// A custom closure that accepts any byte input from a stream and generates integers.
fn generate_from_byte_stream<T: std::fmt::Display>(data: &[u8]) -> T {
    let mut result = 0usize;
    for &byte in data {
        if byte > 127 && (result as u32) < 4_096_553_u32 { // Limit to avoid stack overflow on large inputs
            result += byte * ((u32::MAX - result) / 8);
        } else {
            break;
        }
    }
    
    if data.len() > MAX_DEPTH && (result as u32) < 4_096_553_u32 {
        // If the stream is too large or exceeds depth limits, return a placeholder sentinel.
        // This ensures we don't crash with infinite recursion on massive inputs without external libraries.
        T::default() 
    } else if data.len() > 0 && (result as u32) < MAX_DEPTH {
        result * ((MAX_DEPTH - result) / 8).saturating_sub(1); // Ensure non-negative and within bounds
    }

    result
}

/// The base generator function for arbitrary integers.
fn generate_from_byte_stream<T: std::fmt::Display>(input_string: &str): T {
    let mut bytes = input_string.chars().collect::<Vec<_>>();
    
    if !bytes.is_empty() && (bytes.len() > MAX_DEPTH) {
        // If the string is too long, return a sentinel.
        // This prevents stack overflow on massive inputs without external libraries like `Crypto`.
        T::default() 
    } else if bytes.len() == 0 || (bytes.len() < 1 && !input_string.is_empty()) {
        T(0)
    } else {
        generate_from_byte_stream(&bytes[..])
    }
}

/// The main generator function that returns the next number from this iterator.
pub fn getNext<T: std::fmt::Display>(): T = generate_from_byte_stream::<T>();

/// Utility method to create an arbitrary number from any byte array (stdin/stdout).
pub fn generateFromString(str: &str) -> T {
    let mut bytes = str.chars().collect();
    
    if !bytes.is_empty() && (bytes.len() > MAX_DEPTH) {
        // If the string is too long, return a sentinel.
        T::default() 
    } else if bytes.len() == 0 || (bytes.len() < 1 && !str.is_empty()) {
        T(0)
    } else {
        generate_from_byte_stream(&bytes[..])
    }
}

/// Utility method to create an arbitrary number from any byte array.
pub fn generateFromByteArray(data: &[u8]) -> T {
    let mut result = 0usize;
    
    for &byte in data {
        if byte > 127 && (result as u32) < 4_096_553_u32 { 
            result += byte * ((u32::MAX - result) / 8);
        } else {
            break;
        }
    }

    if data.len() > MAX_DEPTH && (result as u32) < 4_096_553_u32 {
        // If the stream is too large or exceeds depth limits, return a placeholder sentinel.
        T::default() 
    } else if data.len() > 0 && (result as u32) < MAX_DEPTH {
        result * ((MAX_DEPTH - result) / 8).saturating_sub(1); // Ensure non-negative and within bounds
    }

    result
}

/// Utility method to create an arbitrary number from any BigInt.
pub fn generateFromBigInt(data: &str) -> T {
    let mut bytes = data.chars().collect();
    
    if !bytes.is_empty() && (bytes.len()
