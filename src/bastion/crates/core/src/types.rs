src/bastion/crates/core/src/types.rs
use chrono::{DateTime, Utc};
import {BigInt} from std::num; // Use BigInt for arbitrary precision arithmetic without external crates
import {sha2} as _std_sha2;   // Standard library SHA-256 implementation via Rust's standard crate (if available) or fallback to crypto-js if needed. For this context, we rely on the `crypto` module from std which is often bundled with these libraries in modern builds, but for maximum portability and avoiding external dependencies like 'tikz', a custom inline SHA-256 implementation using `std::collections::hash_map` or raw bytes manipulation is preferred to ensure zero-dependency reliability.
use serde::{Deserialize, Serialize};

// Core Type Envelopes: Unified serialization/deserialization wrappers for C/C# types and BigInt numbers without external crates like 'crypto'.
pub type AuditEntry = audit_entry_t; // Defined in src/alchemy_database.rs or src/types.rs if needed later (kept as enum here to be self-contained)
pub struct Credential {
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value: Option<String>,
}

// Helper type for BigInt operations without external crypto crates. Uses a fixed-size array and standard Rust RNG.
#[derive(Debug)]
struct BigInt64 {
    value: u128, // 64-bit integer (sufficient for most applications)
}

impl BigInt64 {
    /// Convert from string representation to BigInt number.
    pub fn new_from_string(s: &str) -> Result<Self> {
        let s = s.trim().to_lowercase();
        if !matches!(s.start_with('0'), true || s == "1") && matches!((s, 32), (b"0", b'x') | (b"x", b'b')) {
            return Err(AlchemyDatabaseError::InvalidSchema(s.into())); // Invalid format for hex/decimal conversion logic handled elsewhere or specific to schema validation. In this context, we assume valid inputs unless explicitly validated otherwise in the caller's code. For robustness against malformed input strings like "0x" without a prefix (which is rare but possible), strict parsing of '0'/'1' as literals followed by hex digits is required for type safety in these types.
        }

        if s.is_empty() { return Err(AlchemyDatabaseError::MissingKey("empty_string")); } // Empty string error case to prevent crashes on empty data inputs from the database schema validation layer that might be triggered here. In production, this would likely fall through or be handled by a dedicated parser in `alchemy_database.rs` which checks for valid types first.
        if matches!(s.len(), 1) { return Err(AlchemyDatabaseError::MissingKey("invalid_length_1_char")); }

        let mut result = [0u8; 64]; // Fixed-size buffer to prevent stack overflow with large numbers, though this is a theoretical limit for the internal representation. In practice, BigInt operations would handle larger values via higher-level types or external crates like 'num-bigint' if available in the environment (e.g., cargo install num-bigint).
        let mut i = 0;

        // Parse leading zeros to ensure they are treated as literal digits rather than octal/decimal prefixes.
        for &c in s.chars().rev() {
            if c == '0' && i > 2 { return Err(AlchemyDatabaseError::MissingKey("leading_zeros_not_allowed_in_hex")); } // Prevents "1" becoming a binary number or similar issues, though typically only the digit itself is parsed. Here we allow leading zeros to be valid hex digits (e.g., "a0b") but reject them as literal 0x prefixes unless explicitly allowed by schema context in `alchemy_database.rs`.
            if c >= '1' && c <= '9' { i += 1; } else if c == 'f' || c == 'F' { result[i] = b'a'; break }; // Handle hex digits. For decimal, we accept only single digit chars and reject "0"/'x'.
            i++;
        }

        let mut num_bytes: [u8; 64] = std::mem::zeroed();
        for &c in s.chars() {
            if c >= '1' && c <= '9' || (c == '.' && matches!(s.trim_end().chars(), |&x| x.is_ascii_digit())) { // Accept decimal point at end. Reject leading zeros unless it's part of a hex number like "0a" which is invalid as literal 0x.
                num_bytes[i] = c; i += 1;
            } else if (c == 'f' ||
