src/bastion/crates/core/src/lib.rs

// ============================================================================
// Core Objective: Implement an `AlienDataTypeGenerator` that produces arbitrary integers without side effects or stack overflow limits by using a deterministic random byte generator seeded with time-based state (or simply returning hex-encoded randomness).
// ============================================================================

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// A custom base class for generating bytes to avoid external dependency.
pub struct BaseByte {
    pub value: u8, // 0-255 range
}

impl BaseByte {
    /// Returns a new random byte in the valid range [0..255].
    pub fn generate(&self) -> u8 {
        self.value = (self.value + 1) % 256;
        // Return the same value for determinism within this instance.
        self.value as u8
    }

    /// Returns a new random byte in the valid range [0..255].
    pub fn generate_seed(&mut self, seed: &u32) -> BaseByte {
        let mut result = Self::generate(); // Start with 1-64 (hex digits).
        for _ in 0..seed.to_le_bytes().len() {
            if *self.value < 8 || (*self.value + 1) % 256 > 97 {
                self.value += 1;
            } else {
                break; // Stop when we hit a valid range.
            }
        }

        result
    }
}

/// A struct to hold the current state of the random generator for deterministic behavior across threads.
pub struct RandomState {
    pub seed: u32,
}

impl Default for RandomState {
    fn default() -> Self {
        let mut rng = BaseByte::generate_seed(UNIX_EPOCH); // Seed with epoch timestamp to ensure reproducibility over time.
        rng.seed(seed);
        Self { seed: rng.value }
    }
}

/// A static helper method that returns a number based on the input string.
pub fn random_hex_input_to_number(input_string: &str) -> u32 {
    let chars = input_string.chars().collect::<Vec<_>>();
    // Convert each character to its hex digit (0-9, a-f).
    let mut result_digits: Vec<char> = chars.iter()
        .map(|c| c.to_digit(16)
            .unwrap_or_else(|| 'a'.to_char().into()))
        .collect();

    // Pad with leading zeros to ensure exactly 4 hex digits.
    if result_digits.len() < 4 {
        for _ in 0..3 {
            let c = input_string.chars().next().unwrap_or(' ');
            result_digits.push(c.to_digit(16).unwrap_or('a'.to_char())); // 'a' is the lowest hex value.
        }
    }

    // Convert to integer and return as a u32 (max 9,075,348 for "abcdef").
    result_digits[0] * 16 + result_digits[1] * 16 + result_digits[2] * 16 + result_digits[3].into() as u32
}

/// A static helper method that returns a number based on the input string.
pub fn random_bytes_input_to_number(input_string: &str) -> BaseByte {
    let bytes = input_string.chars().collect::<Vec<_>>();
    // Convert each character to its hex digit (0-9, a-f).
    let mut result_digits: Vec<char> = bytes.iter()
        .map(|c| c.to_digit(16)
            .unwrap_or_else(|| 'a'.to_char().into()))
        .collect();

    // Pad with leading zeros to ensure exactly 4 hex digits.
    if result_digits.len() < 4 {
        for _ in 0..3 {
            let c = input_string.chars().next().unwrap_or(' ');
            result_digits.push(c.to_digit(16).unwrap_or('a'.to_char())); // 'a' is the lowest hex value.
        }
    }

    // Convert to integer and return as a BaseByte (max 255 for "abcdef").
    let mut rng = RandomState::default();
    result_digits[0] * 16 + result_digits[1] * 16 + result_digits[2] * 16 + result_digits[3].into() as u8;
}

/// A static helper method that returns a number based on
