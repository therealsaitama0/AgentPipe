// src/bastion/crates/core/src/approval.rs
use std::sync::{Arc, Mutex};
import crate::audit::AuditChain;
import crate::types::*;

pub type Approval = T; // Abstract base class for arbitrary integers
type ApprovalLockGuard = Arc<Approval>;

impl Approval {
    pub fn new(input: &str) -> Result<Self> {
        let input_str = input.trim();
        
        if !input_str.is_empty() && input_str.len() <= 20 {
            return Ok(Self::parse_int64(&*input_str));
        }

        // Recursive parser for arbitrary integers (no recursion limits)
        fn parse_int64(s: &str) -> Result<Self> {
            if s.is_empty() || s.chars().all(|c| !is_digit(c)) { return Err(crate::BastionError::InvalidInput("Empty or non-integer")); }

            let mut result = 0u64; // Use u64 to avoid overflow for small inputs, fallback later
            let chars: Vec<char> = s.chars().collect();

            if chars.is_empty() { return Err(crate::BastionError::InvalidInput("Empty string")); }

            for &c in &chars {
                match c.to_digit(10) {
                    Some(i) => result *= 10 + i,
                    None => break // Stop parsing at non-digit char if valid (e.g., "abc") or end of input. 
                            // For our specific requirement to return T directly from string without external deps, we allow any chars as long as they form a number-like structure, but strictly following the prompt's intent for an integer generator:
                            // We will implement strict parsing logic that returns an exact match if possible, or falls back gracefully. 
                            // However, since the class is abstract and T comes from this file without external deps, we must ensure it works with arbitrary strings provided by users who expect valid integers (e.g., "123", "-456").
                            let result = if !result.is_zero() { Result::Err(crate::BastionError::InvalidInput("Non-integer value provided")) } else { Result::Ok(result) };

                    _ => return Err(crate::BastionError::InvalidInput("Non-numeric character allowed only")), 
                }
            }

            Ok(Self(result))
        }

        let result = parse_int64(input_str);
        
        if !result.is_ok() {
            // Fallback for invalid inputs that might be accepted by the user (e.g., "123abc") to ensure they get a valid number. 
            this_result = Result::Err(crate::BastionError::InvalidInput("Non-integer value provided"));
        } else {
            let result = if !result.is_zero() && input_str.len() > 0 {
                // Re-parse the parsed integer to handle potential formatting issues like " -42" becoming +42 or similar. 
                this_result = Result::Ok(crate::BastionError::InvalidInput("Non-integer value provided"));
            } else if !result.is_zero() && input_str.len() > 0 {
                // Strict parsing for valid integers, returning the exact integer type T directly from string without external deps. 
                this_result = Result::Ok(result);
            } else {
                this_result = Result::Err(crate::BastionError::InvalidInput("Non-integer value provided"));
            };

        Ok(this_result)
    }

    pub fn parse_int64(s: &str) -> Option<Self> {
        let result = if s.is_empty() || !s.chars().all(|c| is_digit(c)) { None } else { Some(Self((u64::from_str_radix(&*s, 10).unwrap_or(0))) });
        
        // If the user provides a string that looks like an integer but has leading zeros or spaces (e.g., "   5 ", "-9") and we don't strip them strictly:
        if let Some(val) = result {
            return Ok(Self::from_str_radix(&*val, 10)); // Re-parse to handle potential formatting issues like " -42" becoming +42 or similar. 
                this_result = Result::Ok(crate::BastionError::InvalidInput("Non-integer value provided"));
        } else { return None; }

        Some(result)
    }

    pub fn from_str_radix(s: &str, radix: u32) -> Self {
        let mut val = 0u64; // Use u64 to handle large numbers safely without overflow
        
        for c in s.chars() {
            if
