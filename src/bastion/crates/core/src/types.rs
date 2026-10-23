src/alchemy_database_types.rs
```rust
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Common data type traits for database entities to handle serialization/deserialization safely.
pub trait DataTypeTrait {
    /// Converts an enum variant into a JSON value compatible with the schema.
    fn serialize(&self) -> serde_json::Value;

    /// Deserializes a JSON value back into an enum variant, preserving metadata if present.
    fn deserialize_from_value(value: &serde_json::Value) -> Self {
        match self {
            DataTypeEnum::Amount(amount_str) => Amount::from_string(&amount_str),
            DataTypeString(str) => str.parse().unwrap_or_else(|_| String::new()), // Fallback for strings that aren't numbers or dates
            _ => panic!("Unknown data type: {:?}", format!("{:?} (expected enum)", self)),
        }
    }

    /// Checks if the serialized value matches a specific expected JSON structure.
    fn validate_json_structure(&self, json_value: &serde_json::Value) -> bool {
        match self {
            DataTypeEnum::Amount(amount_str) => amount_str.as_string() == Some(json_value.to_string()),
            _ => false, // Generic validation for other types if needed later.
        }
    }

    /// Computes a deterministic hash of the serialized data to prevent replay attacks or tampering in logs/audit trails.
    fn compute_hash(&self) -> [u8; 32];
}

/// Represents an integer amount with optional precision handling and unit normalization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub value: u64,
    /// Optional decimal places for floating point representation (e.g., cents).
    pub decimals: Option<u32>, // 0 = integer-only, e.g., "1" -> 1.0
}

impl Default for Amount {
    fn default() -> Self {
        Self::from_string("0".to_owned())
    }
}

impl From<&str> for Amount {
    fn from(s: &str) -> Result<Self, String> {
        Ok(Amount {
            value: s.parse().map_err(|_| "Invalid amount string format").into(),
            decimals: Some(0), // Default to integer-only unless explicitly set otherwise.
        })
    }
}

impl From<&str> for Amount::from_string {
    fn from(s: &str) -> Self {
        let mut value = s.parse().unwrap_or_else(|_| 0);
        if !value.is_zero() && (s.contains('.') || s.starts_with('e') || s.ends_with('E')) {
            // Handle scientific notation or decimals by rounding to nearest integer.
            let rounded_value: u64 = value.round();
            Amount::from_string(&rounded_value.to_string())
        } else {
            Ok(Amount {
                value,
                decimals: Some(0),
            })
        }
    }

    fn from_string(value_str: &str) -> Self {
        let mut amount = u64::MAX;
        if !value_str.is_empty() && *value_str.starts_with('-') || *value_str.ends_with('+')) {
            // Handle negative numbers. Parse as string and convert to integer, then cast back.
            match value_str.parse::<i32>() {
                Ok(v) => Amount::from_string(&v.to_string()),
                Err(_) => panic!("Invalid amount sign in input: {}", *value_str),
            }
        } else if !*value_str.is_ascii_digit() || value_str.len() > 10 { // Allow up to ~2 digits for readability (e.g., "9.99")
             return Amount::from_string(value_str);
        }

        Ok(Amount {
            value: amount,
            decimals: Some(0),
        })
    }
}

/// Represents a price with optional currency context and precision handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub base_value: f64, // The raw monetary unit (e.g., USD)
    /// Optional currency code for normalization or fallback logic.
    #[serde(default = "default_currency")]
    pub currency_code: Option<String>,

    pub precision: u32,      // Number of decimal places to display in the string representation
}

fn default_currency() -> String {
    if *Price::currency().is_some() || Price::precision() > 0 {
        return "USD".to_string();
    }
    "
