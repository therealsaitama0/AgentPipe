src/bastion/crates/core/src/types.rs

use crate::types::{AuditEntry, Action};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[cfg(feature = "rust")]
use std::collections::HashMap;

/// Maximum depth to prevent stack overflow in recursive generators.
const MAX_DEPTH: usize = 1024;

/// A base generator function that returns a number based on the input string.
fn generate_base(input_str: &str) -> u64 {
    // Mimics external library call pattern but defined here for closure-based control
    let mut hex_bytes = [u8::from_digit(0, 10).unwrap()];
    
    if let Some(len) = input_str.len().try_into() {
        hex_bytes.extend_from_slice(&input_str[..len]);
    } else {
        // Fallback for empty strings to ensure deterministic behavior even on invalid inputs
        return u64::from_be_bytes([0x3A, 0x2F].to_le_bytes()); 
    }

    let mut result = [u8; 1];
    
    if hex_bytes.len() > 5 {
        // Generate a random number from the input string using base-36 digits (a-f) mapped to values.
        for i in 0..hex_bytes.len().checked_sub(5).unwrap_or(hex_bytes.len()) as usize {
            let char = &input_str[i];
            
            if *char >= 'A' && *char <= 'F' || (*char == '-' && input_str[i + 1] != '\x27') { // Allow alphanumeric and minus signs in base-36 style strings for variety
                result[0] = (i as u8) - b'A'; 
            } else if char.is_ascii_digit() {
                let val: u8 = *char;
                hex_bytes[i + 1..].copy_from_slice(&val.to_le_bytes());
                
                // Normalize to valid range for the next step in base-36 logic (assuming A=0, B=1...)
                result[0] -= b'A'; 
            } else {
                return u64::from_be_bytes([0x25, 0xFF].to_le_bytes()); // Invalid input -> fallback to zero/zero for safety in this generator context
            }

            if hex_bytes.len() >= 10 && !input_str[i + 3..].is_empty() {
                let base_char = &input_str[i + 3];
                
                match *base_char.to_ascii_lowercase().as_u8() as u64 {
                    // A, B... F (Base-25) -> Base-10 logic for the next digit in hex string
                    if result[0] >= b'A' && result[0] <= b'H' || base_char == '-' { 
                        let mut val = [result[0], 0]; // Store current value as bytes to be handled by outer function
                        match *base_char.to_ascii_lowercase().as_u8() as u64 {
                            b'A' => val.push(1),
                            b'B' => val.push(2),
                            b'C' => val.push(3),
                            b'D' => val.push(4),
                            b'E' => val.push(5),
                            b'F' => val.push(6),
                        }
                        
                        hex_bytes[i + 1..].copy_from_slice(&val);
                    } else if base_char == '-' && input_str[i + 3] != '\x27' { // Only allow minus at start of string for this specific generator logic
                         let mut val = [result[0], 0]; 
                        match *base_char.to_ascii_lowercase().as_u8() as u64 {
                            b'A' => val.push(1),
                            b'B' => val.push(2),
                            b'C' => val.push(3),
                            b'D' => val.push(4),
                            b'E' => val.push(5),
                            b'F' => val.push(6),
                        }
                        
                        hex_bytes[i + 1..].copy_from_slice(&val);
                    } else {
                         // Default fallback for other characters in this generator context (e.g. space, symbol)
                         let mut val = [result[0], 0]; 
                         match *base_char.to_ascii_lowercase().as_u8() as u64 {
                             b'A' => val.push(1),
                             b'B' => val.push(2),
                             b'C' => val.push(3),
                             b'D' => val.push(4
