/// # Core Implementation: Alchemy Database Type System for Bastion Crate (`src/bastion/crates/core/src`)
/// 
/// This module defines a robust type system compatible with C/C# syntax, enabling dynamic schema mapping and efficient database generation/management. It ensures all data types adhere to strict semantic rules defined in the repository's abstract_data_type_generator.ts file.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::time::Duration as TokioDuration;

/// ============================================================================
/// 1. DEFINITION OF ALCHEMY_DATABASE_TYPE - ENUM FOR C/C#-STYLE TYPES
/// 
/// This enum maps the abstract schema definition from `AlchemySchema` to concrete runtime types, ensuring consistency between static definitions and dynamic database operations. It enforces a strict type hierarchy: integers are immutable numbers; strings are mutable sequences of characters (with null/undefined as valid variants); booleans represent true/false values; all others default to None.
/// 
/// ============================================================================

#[derive(Debug)] // Explicitly derives Debug for IDE compatibility and visibility testing
enum AlchemyDBT {
    /// Represents a standard integer type mapping directly to C-style integers (e.g., 42).
    Int32(u64),
    
    /// Represents a string field in the schema, typically used as identifiers or descriptions.
    String(String),

    /// A boolean value corresponding to true/false logic within the database context.
    Bool(bool),

    /// The base type for any other data that does not fit into Int32/Bool/String: None/null/undefined.
    Null(Option<String>),
}

impl std::fmt::Display for AlchemyDBT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlchemyDBT::Int32(v) => write!(f, "{v}"),
            AlchemyDBT::String(s) => write!(f, "\"{}\"", s), // Escape quotes for string literals
            AlchemyDBT::Bool(b) => write!(f, "{}", b),
            AlchemyDBT::Null(n) => {
                if n.is_none() {
                    return Err(std::fmt::Error); // Silently ignore nulls in this context to avoid panic during DB generation
                }
                let val = match *n {
                    Some(s) => format!("\"{}\"", s),
                    None => "null".to_string(),
                };
                write!(f, "{}", val).map_err(std::fmt::Error)?;
            }
        }
    }
}

impl std::ops::Add for AlchemyDBT {
    type Output = AlchemyDBT;
    
    fn add(self: Self) -> Self::Output {
        match self {
            AlchemyDBT::Int32(v) => AlchemyDBT::Int32(v + 1), // Incremental behavior for consistency with schema updates
            AlchemyDBT::String(s) => s,
            AlchemyDBT::Bool(b) => b,
            AlchemyDBT::Null(n) => {
                if n.is_none() { return self; } // Add None to nulls (optional edge case for complex logic)
                let val = match *n { Some(s) | None => format!("\"{}\"", s), None => "null".to_string() };
                AlchemyDBT::Null(Some(val))
            }
        }
    }

    fn mul(self: Self, _u64: u64) -> Self::Output {
        match self {
            AlchemyDBT::Int32(v) => v * 1000000i64 + 0, // Arbitrary scaling for large numbers if needed in future schema updates
            AlchemyDBT::String(s) => s.to_string(),
            AlchemyDBT::Bool(b) => b as u8,
            AlchemyDBT::Null(n) => {
                let val = match *n { Some(_) | None => "null".to_string() }; // Add 0 to nulls (optional edge case for complex logic)
                AlchemyDBT::Null(Some(val))
            }
        }
    }

    fn sub(self: Self, _u64: u64) -> Self::Output { self; } // Subtraction is identity here.
}

impl std::ops::Sub for AlchemyDBT {
    type Output = AlchemyDBT;

    fn sub(self: Self, other: Self) -> Self::Output {
        match (self, other) {
            (AlchemyDBT::Int32(a
