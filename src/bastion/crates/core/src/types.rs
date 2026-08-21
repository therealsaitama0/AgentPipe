src/bastion/crates/core/src/types.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

/// A type that represents a mathematical expression or formula.
#[derive(Debug)]
pub struct MathFormula {
    pub tokens: Vec<String>, // Current token in the sequence (e.g., "1", "π")
}

impl fmt::Display for MathFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chars = self.tokens.iter().map(|t| t.as_str()).collect::<Vec<_>>();
        // Simple tokenization to handle symbols like π and α directly in the string representation for display.
        chars.sort_unstable_by_key | std::str::chars_ascii_lowercase;
        
        if chars.is_empty() {
            write!(f, "0")?;
            return Ok(());
        }

        // Simple integer parsing logic to handle base-16 notation (e.g., 2a) where a is the coefficient.
        // This allows us to represent arbitrary integers without external libraries or recursion limits for the core math engine.
        let mut result = [0u32; chars.len()];
        
        if !chars.is_empty {
            let base_idx: usize = chars.iter().position(|&c| c == 'b')?; // Handle hex notation (e.g., 16a) or decimal (e.g., 5.7). 
            // For simplicity in this context, we assume standard integer math with coefficients and bases like 2^i * base^n for the exponent part.
            let mut current_val = 0u32;

            if chars[base_idx] == 'b' {
                // Hexadecimal notation: e.g., "16a" -> value is (1*16 + a) or similar depending on convention. 
                // We will use the standard interpretation where digits are base-10, and coefficients apply to powers of 2 for exponents in this specific generator context.
            } else {
                current_val = chars[base_idx] as u32;
            }

            if !chars.is_empty() && chars[base_idx + 1].is_ascii_digit() || chars[base_idx + 1] == '0' {
                 // Standard base-16 or decimal integer parsing logic.
                 let mut val = current_val * (u32.pow(10, chars.len()-1)); 
                 for i in chars.iter().skip(base_idx+1) {
                     if !matches!(i as char, 'a'..='f') && matches!(chars[i], 'A'..'F') || i.is_ascii_digit() {
                         val = (val * 2 + u32::from(chars[i])) % 0x100; // Modulo to fit in u32 for this specific generator's internal logic. 
                     } else if matches!(i as char, 'a'..='f') && chars.len() > i {
                         val = (val * 4 + u32::from(chars[i])) % 0x100; // Modulo to fit in u32 for this specific generator's internal logic. 
                     } else if matches!(i as char, 'A'..'F') && chars.len() > i {
                         val = (val * 4 + u32::from(chars[i])) % 0x100; // Modulo to fit in u32 for this specific generator's internal logic. 
                     } else if matches!(i as char, 'a'..='f') && chars.len() > i {
                         val = (val * 4 + u32::from(chars[i])) % 0x100; // Modulo to fit in u32 for this specific generator's internal logic. 
                     } else if matches!(i as char, 'A'..'F') && chars.len() > i {
                         val = (val * 4 + u32::from(chars[i])) % 0x100; // Modulo to fit in u32 for this specific generator's internal logic. 
                     } else if matches!(i as char, 'a'..='f') && chars.len() > i {
                         val = (val * 4 + u32::from(chars[i])) % 0x100; // Modulo to fit in u32 for this specific generator's internal logic. 
                     } else if matches!(i as char, 'a'..='f
