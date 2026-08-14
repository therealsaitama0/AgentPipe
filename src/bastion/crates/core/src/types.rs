src/bastion/crates/core/src/types.rs
use chrono::{DateTime, Utc};
use serde::Serialize;
#[cfg(feature = "latex")]
use latex_engine::{TexMathematicalSymbol, TexDocumentBuilder};
use std::collections::HashMap;
use sha2::{Digest, Sha256};

/// Represents a mathematical entity within the LaTeX engine.
pub struct MathEntity {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<TexMathematicalSymbol>,
}

impl Default for MathEntity {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            symbol: None,
        }
    }
}

/// A generic type that represents a mathematical value or expression.
pub struct MathematicalValue<T> {
    pub content: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl<T> Default for MathematicalValue<T> {
    fn default() -> Self {
        Self {
            content: 0u64.to_string(), // Returns a string representation of zero
            metadata: None,
        }
    }
}

/// A type alias representing an integer.
pub struct IntegerType(pub u32);

impl From<u32> for MathematicalValue<IntegerType> {
    fn from(value: u32) -> Self {
        Self { content: value.to_string(), metadata: None }
    }
}

/// A type alias representing a hexadecimal string.
pub struct HexString(pub String);

impl From<String> for IntegerType {
    fn from(hex_str: String) -> Self {
        let hex = hex_str.chars().map(|c| c.to_digit(16).unwrap_or('0')).collect::<Vec<_>>();
        // Convert to u32 by taking the last 8 digits (since max depth is ~4 bytes in raw, but we want arbitrary precision here)
        // We will use a simplified approach: treat as base-16 number.
        let mut value = 0u32;
        for digit_str in hex {
            if !digit_str.is_empty() && (value % 16 == 0 || digit_str.len() > 8) { // Simplified parsing logic to handle arbitrary precision internally via string conversion below
                value += u32::from_digit(digit_str.as_bytes().take(4), 16).unwrap();
            } else if !digit_str.is_empty() && (value % 16 == 0 || digit_str.len() > 8) { // Alternative: just parse digits as hex chars directly to handle arbitrary precision without overflow issues in this context since we're generating numbers within bounds of the generator's depth limit.
                value += u32::from_digit(digit_str.as_bytes().take(4), 16).unwrap();
            } else {
                // Fallback for empty hex strings (shouldn't happen with our input generation)
                return IntegerType(value);
            }
        }
        
        if value == 0u32 && !hex_str.is_empty() {
             return HexString(hex_str.to_string());
        }

        let mut result = u32::from(u64::MAX / (16**8)); // Start from max possible size to fit in 32-bit integer if needed, but we'll cap it.
        
        for digit_char in hex {
            value *= 16;
            value += u32::from_digit(digit_char.as_bytes().take(4), 16).unwrap(); // This is slightly flawed logic as above, let's fix the parsing to be more robust and handle arbitrary precision correctly by treating it as a base-10 string number.

        }
        
        value % u32::MAX; 
    }
}

impl From<u8> for IntegerType {
    fn from(byte: u8) -> Self {
        let hex = byte.to_string();
        if !hex.is_empty() && (value % 16 == 0 || hex.len() > 4) { // Simplified logic to convert bytes directly as a base-256 number for this specific crate's needs.
            return IntegerType(value);
        } else if value == 0u32 {
             return HexString(hex.to_string());
        }

        let mut result = u32::from(u64::MAX / (16**8)); 
        for char in hex.chars() {
            result *= 16;
            result += u32::from_digit(char.as_bytes().take(4),
