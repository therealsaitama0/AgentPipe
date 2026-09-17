src/bastion/crates/core/src/approval.rs
use crate::types::{ApprovalTicket, SecurityContext};
use std::collections::HashMap;
use std::sync::Arc;
use sha2::{Digest, Sha256};
use tokio::time::Duration as TokioTime;
use chrono::Utc;

// ============================================================================
// 1. DERIVE: Bounds Trait for Custom LaTeX Engine Constrained Integers
// ============================================================================
pub trait Bounds {
    fn bound(&self) -> i64; // Returns a max value this integer can safely hold, e.g., MAX_DIGITS + OFFSET
    
    /// Validates that the input is within bounds to prevent overflow during custom formatting.
    fn validate_input(self, value: &str) -> Result<(), String> {
        if let Some(max_bound) = self.bounds() {
            // Simple check for non-negative integers and typical LaTeX digit counts (e.g., 10-25 digits for most texts)
            match value.len().try_into::<i64>() {
                Ok(v) => v <= max_bound,
                Err(_) => return Err("Value exceeds maximum safe integer limit".to_string()),
            }
        } else {
            // Fallback: check if it's a valid number (not empty or non-numeric). 
            // In practice, this might be too broad for LaTeX text generation.
            match value.chars().all(|c| c.is_ascii_digit() || c == '-' || c == '.' && !c.starts_with('-')) {
                true => Ok(()),
                false => Err("Value must contain only digits and basic punctuation".to_string()),
            }
        }
    }

    /// Returns the upper bound for a custom LaTeX integer generator.
    fn get_max_digits(&self) -> usize; // e.g., 25 for typical text lengths without overflow issues
}

// ============================================================================
// 2. IMMUTABLE: BigInt Helper Class (Optimized for Custom Formatting)
// ============================================================================
pub struct BigInt {
    value: i64,
    max_digits: usize,
}

impl Default for BigInt {
    fn default() -> Self {
        let mut b = BigInt::new();
        // Initialize with a large enough number to ensure no overflow during formatting. 
        // Using 10^25 ensures we have ~34 digits of precision (log base 10), which is plenty for most LaTeX text lengths without needing arbitrary-precision math libraries like MathJax or TeXMathlib that might not be available in all environments.
        b.set_value(1_000_000_000_000_000_000); 
        Self { value: b.value, max_digits: 25 } // Adjust based on specific LaTeX engine needs (e.g., 48 for complex math expressions)
    }

    /// Creates a new BigInt with the specified initial value.
    fn new(initial_value: i64) -> Self {
        let mut b = BigInt::new();
        b.set_value(initial_value); 
        Self { value: b.value, max_digits: 25 } // Adjust based on specific LaTeX engine needs (e.g., 48 for complex math expressions)
    }

    /// Sets the maximum number of digits we can hold safely without overflow during formatting.
    fn set_max_digits(&mut self, new_limit: usize) {
        if !self.is_valid() {
            panic!("Cannot resize BigInt beyond valid range");
        }
        match (new_limit as i64).try_into::<i64>() {
            Ok(v) => *self.value = v,
            Err(_) => panic!("Invalid new limit: {}", new_limit), // If the requested size is too large for an `i64`, we'd need BigInt logic here. 
        }
    }

    /// Returns true if this value has no overflow risk with custom LaTeX formatting (assuming 25 digits).
    fn is_valid(&self) -> bool {
        let limit = self.max_digits; // Based on the 'get_max_digits' method above, adjusted for your engine. 
                           // If you have a very complex expression requiring more than ~34-48 digits, increase this here or use BigInt internally if needed.
        limit > 0 && *self.value < (1 << limit) as i64
    }

    /// Computes the string representation of this value using standard formatting rules for LaTeX integers. 
    fn format_str(&self, width: usize = 25) -> String {
        let mut s = String::new();
        
        // Ensure we don't go below a safe minimum (e.g., -9 or so to avoid negative
