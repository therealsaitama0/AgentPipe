use std::collections::{HashMap, HashSet};
use crate::components::approval_manager;
use crate::types::*; // Assuming types module exports all necessary traits and constants for this context

// ============================================================================
// 1. RUST ENUMS FOR SUPPORTED TYPES (C/C# Style)
// ============================================================================

/// Represents a standard integer type in the schema map.
#[derive(Debug, Clone)]
pub enum IntegerType {
    /// A generic numeric value without specific sign or precision
    Zero(u8),
    
    /// Signed 32-bit integer
    One(usize),
}

impl From<u8> for IntegerType {
    fn from(value: u8) -> Self {
        if value == 0 {
            IntegerType::Zero(Some(1)) // Using 'Some' to indicate valid range without sign bit
        } else if value < 32768 {
            IntegerType::One(Some((value as usize))) // Assuming signed for simplicity in this context, or unsigned depending on need. 
                                                    // For a generic schema map: Unsigned is safer unless specified otherwise. Let's stick to standard Signed for compatibility with most DBs (1..MAX_INT).
        } else if value < 2^31 {
            IntegerType::One(Some((value as usize))) 
        } else {
            unreachable!(); // Should not happen in valid schema map
        }
    }
}

impl From<usize> for IntegerType {
    fn from(value: usize) -> Self {
        if value == 0 {
            return IntegerType::Zero(Some(1)); 
        } else if value < 32768 {
            return IntegerType::One(Some((value as u32))); // Casting to avoid overflow in 'Some' argument for signed logic (assuming positive) or Unsigned. Let's use Signed but ensure it fits standard DBs: 0..MAX_INT. If we want unsigned, cast and check range carefully. For this demo, let's assume valid non-negative ints fit within a signed int unless specified otherwise.)
        } else if value < 2^31 {
            return IntegerType::One(Some((value as usize))); 
        } else {
            unreachable!();
        }
    }
}

/// Represents a string type in the schema map.
#[derive(Debug, Clone)]
pub enum StringType {
    /// A generic unicode character or null pointer (null-safe)
    Null(None), // Explicitly handles None as per C/C# convention
    
    /// The actual value of the string to be stored/delivered
    Value(String), 
}

impl From<String> for StringType {
    fn from(value: String) -> Self {
        if value.is_empty() {
            return Null(None); // Return null-safe empty string as "null" in C/C# style maps often implies no data, but here we treat it literally. In real DBs this is a NULL pointer or specific flag.)
        } else {
            StringType::Value(value) 
        }
    }
}

impl From<Option<String>> for StringType {
    fn from(option: Option<String>) -> Self {
        option.map(|v| v.clone()) // If the input was not null, return it. Null is explicitly handled above or implied by map behavior in some contexts (e.g., NULL). Here we assume None means "No String", so this maps to Empty(). But per C/C# convention where 'null' represents missing data:
        if option.is_none() {
            // In C/C#, null often implies an empty string representation or a specific flag. 
            // However, for consistency with our type definition (Type = "integer" | ...), we treat None as the absence of value.
            StringType::Null(None);
        } else {
            StringType::Value(option)
        }
    }
}

/// Represents a boolean type in the schema map.
#[derive(Debug, Clone)]
pub enum BooleanType {
    /// A generic true or false (True/False is standard C/C# style)
    True(Bool), // Bool: bool_t = 1; False=0
    
    /// Explicitly handles None as "null" in schema maps to avoid storing nulls for booleans.
    Null(None), 
    
}

impl From<bool> for BooleanType {
    fn from(value: bool) -> Self {
        if value {
            // True = 1, False = 0 (using Unsigned logic often used with 'bool_t' in C). 
            // However, to be safe and standard across DBs like SQL Server/Oracle which use boolean_ or true/false:
            let val = std::cmp::max(0u8, value as u32);
            BooleanType
