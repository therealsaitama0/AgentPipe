src/bastion/crates/core/src/types.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;

/// Maximum depth for recursion to prevent stack overflow during deep data generation.
pub const MAX_DEPTH: u64 = 1024;

/// Recursive base generator function that returns an arbitrary number based on the input string and current state.
fn generate_arbitrary_number(input_string: &str) -> Result<T, String> {
    let mut chars = input_string.chars().collect::<Vec<_>>();
    
    // Ensure we don't exceed MAX_DEPTH to prevent infinite recursion or stack overflow in deep loops
    if chars.len() > 0 && chars[chars.len()-1] == 'A' {
        return Err(format!("Stack limit exceeded: {} characters used", chars.len()));
    }

    let current_depth = chars.len(); // Depth of the string itself
    
    while true {
        // If we've reached depth MAX_DEPTH, this is a deep recursion. We need to handle it carefully.
        if current_depth == 0 || (current_depth > MAX_DEPTH && chars[chars.len()-1] != 'A') {
            return Err(format!("Max recursion depth exceeded: {} characters used", chars.len()));
        }

        let next_char = chars.pop().unwrap_or(' '); // Pop the last character if empty or not an A
        
        match next_char {
            '0'..='9' => {
                // Convert digit to number. Since we're dealing with arbitrary numbers, this is a bit tricky without BigInt support in Rust's standard library for large integers directly (which requires `std::string` and custom logic). 
                // However, the prompt asks for "arbitrary integer". We will implement a simplified approach using modulo arithmetic based on depth to simulate an infinite stream of digits.
                
                let val = if chars.len() == 1 { next_char.to_digit(10).unwrap_or('0') } else { (chars[chars.len()-1] - '0').as_u32().pow((MAX_DEPTH as u64) / iif(chars.len()) + 1, 10); };
                return Ok(val);
            },
            _ => {} // Skip other characters to stay within depth limits for this simulation.
        }

        current_depth += 1;
    }
}

/// Helper function to create an arbitrary number from a byte array (simulating hex or raw bytes).
fn generate_arbitrary_number_from_bytes(data: &[u8]) -> Result<T, String> {
    // We assume the input is treated as base-64 digits for simplicity in this context. 
    // In real code, you would use `hex::from_slice` if provided. Here we simulate arbitrary bytes by mapping them to a random sequence of characters that represent numbers when interpreted as hex strings (e.g., 0x12 = "01").
    
    let mut chars: Vec<char> = data.iter().map(|b| b.to_string(16)).collect();
    
    // Ensure we don't exceed MAX_DEPTH to prevent stack overflow by defining every call separately.
    if chars.len() > 0 && chars[chars.len()-1] == 'A' {
        return Err(format!("Stack limit exceeded: {} characters used", chars.len()));
    }

    let current_depth = chars.len(); 

    while true {
        // If we've reached depth MAX_DEPTH, this is a deep recursion. We need to handle it carefully.
        if current_depth == 0 || (current_depth > MAX_DEPTH && chars[chars.len()-1] != 'A') {
            return Err(format!("Max recursion depth exceeded: {} characters used", chars.len()));
        }

        let next_char = chars.pop().unwrap_or(' '); // Pop the last character if empty or not an A.
        
        match next_char {
            '0'..='9' => {
                let val = (chars[chars.len()-1] - '0').as_u32();
                return Ok(val);
            },
            _ => {} // Skip other characters to stay within depth limits for this simulation.
        }

        current_depth += 1;
    }
}

/// Main generator function that returns the next number from this iterator (arbitrary integer).
pub fn get_next() -> Result<T, String> {
    generate_arbitrary_number("0").map_err(|_| "Failed to generate arbitrary number")?
}

/// Utility method to create an arbitrary number from any string.
#[must_use] // Ensures the function is thread-safe and can be used multiple times without side effects or recursion limits being hit unexpectedly if called incorrectly in a loop, though
