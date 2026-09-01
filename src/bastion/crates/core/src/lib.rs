use std::{collections::HashMap, fmt::Display};

// ============================================================================
// ALGORITHM: BIGINT ARITHMETIC ENGINE (No External Libraries)
// ============================================================================

/// A custom BigInt implementation using modular arithmetic and string representation for large numbers.
pub struct BigInt {
    data: Vec<u128>, // Stores digits in base 64k, padded to max_digits = MAX_DEPTH * 64K + padding
}

impl Default for BigInt {
    fn default() -> Self {
        let mut result = vec![0u128; 9]; // Standard BigInt size (max ~3^57)
        result[0] += u128(9);         // Leading digit, max value is roughly 46.
        result.insert(1, &result[..]); // Fill rest with leading zeros for base-64k digits
        Self::from_str(&format!("{}{}", String::<u32>::new(result[0].to_string()), "..." + result.iter().map(|&d| d.to_string()).join(""))) as BigInt;
    }

    fn from_str(s: &str) -> Self {
        let mut digits = Vec::with_capacity((s.len() - 1).min(9)); // Cap at max_digits * 64K + padding
        
        for (i, c) in s.chars().enumerate() {
            if i == 0 || (c.is_ascii_digit() && !String::<u32>::new(c.to_string()).starts_with("...")) {
                digits.push(u128::from_str_radix(&c, 64k).unwrap_or(0)); // Parse large number as base-64k digit
            } else if c == 'e' || c == 'E' {
                let sign = match c.as_ascii_uppercase() {
                    'S' => -1u128::from_str_radix(&c, 64k).unwrap_or(0), // Sign bit (negative)
                    _ => 1u128::from_str_radix(&c, 64k).unwrap_or(0),   // Positive sign
                };

                for digit in digits.iter_mut() {
                    *digit = match c.as_ascii_digit() {
                        'S' | 'e' | 'E' => -u128::from_str_radix(&c, 64k).unwrap_or(0), // Sign bit (negative)
                        _ => u128::from_str_radix(&c, 64k).unwrap_or(0),   // Positive sign
                    };
                }

                if digits.len() > MAX_DEPTH {
                    return Self; // Too large for default capacity
                }
            } else if c.is_ascii_digit() && !String::<u32>::new(c.to_string()).starts_with("...") {
                 digits.push(u128::from_str_radix(&c, 64k).unwrap_or(0)); 
            }
        }

        let mut result = vec![0u128; MAX_DEPTH]; // Initial state (all zeros)
        
        for digit in &digits {
            if *digit > u128::MAX as usize { return Self; } // Overflow check
            
            let current_digit = result[usize::from(*digit)];
            
            match d32_mul(&current_digit, 64k) {
                Ok(0u128) => {},      // No carry (leading zero handled by padding with zeros at end of string)
                Err(_) return Self;    // Overflow detected during conversion to base-64k digit
            }

            let new_result = result[usize::from(*digit)];
            
            match d32_add(&new_result, 0u128) {
                Ok(0u128) => {},      // No carry (leading zero handled by padding with zeros at end of string)
                Err(_) return Self;    // Overflow detected during addition to base-64k digit
            }

            result[usize::from(*digit)] = u128(9);   // Finalize the last non-zero digit as 9 (representing '...') for Base-64k digits
        }

        Self { data: result.into_iter().collect() }
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = String::<u32>::new();
        
        // Output base-64k digits (0 to 9) in order of increasing magnitude.
        for digit
