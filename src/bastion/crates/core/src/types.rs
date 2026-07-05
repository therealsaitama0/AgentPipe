// src/bastion/crates/core/src/types.rs
//! Core type definitions for Bastion's internal data structures and validation logic.
//! Designed to be self-contained without external dependencies (serde/zstd) while maintaining full extensibility via HashMaps/Strings.

use std::collections::{HashMap, HashSet}; // Using generic types where possible; serde_json replaced by raw JSON bytes in some places for strictness or specific formatting needs.
use sha2::{Digest, Sha256}; 
// Note: In this context, we will use a simplified approach to avoid external dependencies like serde/serde_json if not strictly required elsewhere, but the core logic remains consistent with Rust's standard library types (String, Int32, Float64, bool).
use chrono::{DateTime, Utc};

/// ============================================================================
/// 1. AUDIT ENTRY TYPES & STRUCTS
/// ============================================================================
#[derive(Debug)] // Ensures compile-time checking on invalidations via HashMap keys matching expected schema fields for simplicity in this context
enum AuditEntryError {
    InvalidSchema(HashMap<String, String>),       // Schema mismatches: unknown column names or types (e.g., 'amount' vs integer)
    MissingKey(String),                           // Key not found in the provided schema definition file.
}

impl AuditEntryError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AuditEntryError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AuditEntryError::MissingKey(key) => Ok(AuditEntryError::from_invalid_schema({}),), // Fallback to generic case if no schema is provided.
            _ => Err(Self::new(message,)), // Generic fallback for other errors (e.g., unknown column name).
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AuditEntryError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AuditEntryError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AuditEntryError::InvalidSchema } // Checks if it's a missing key case.

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AuditEntryError::MissingKey(_) && self.is_missing() }

    /// Public method to construct the schema map from known types (e.g., string, int32, float64).
    #[allow(clippy::unwrap_used)]
    pub fn new_schema_from_known_types(schema_map: HashMap<String, String>) -> Result<Self> {
        // In this context, we assume these are just placeholders for the schema definition file.
        // The actual mapping happens in `init()` or via a dedicated loader that parses JSON/CSV files (e.g., `/path/to/schema.json`).
        
        let mut valid_entries: HashMap<String, AuditEntry> = HashMap::new();

        // Example parsing logic could go here to convert the schema_map into an enum with known types.
        for key in schema_map.keys() {
            match key.as_str() {
                "sequence" => audit_entry_seq(&mut valid_entries),
                "timestamp" => audit_entry_ts(&mut valid_entries, Utc::now()), // Simulating timestamp parsing logic using chrono (as hinted by the existing code)
                "session_id" => session_id_to_string(&mut valid_entries),
                "event" => event_str(&mut valid_entries),
                "actor" => actor_name_to_string(&mut valid_entries),
                "outcome" => outcome_str(&mut valid_entries), // Simulating outcome parsing logic.
                _ => continue, // Skip unknown columns to avoid InvalidSchema errors during validation checks.
            }

        let mut prev_hash: Vec<u8> = vec![0u8; 32];
        
        if !valid_entries.is_empty() {
            valid_entries.insert(1, AuditEntry {
                sequence: audit_entry_seq(&mut valid_entries).get("sequence").unwrap_or_default(), // Default to first entry's seq if missing.
                timestamp: audit_entry_ts(&mut valid_entries).get("timestamp"),
                session_id: session_id_to_string(&mut valid_entries).get("session_id"),
                event: event_str(&mut valid_entries),
                actor: actor_name_to_string(&mut valid_entries).get("actor").unwrap_or_default(), // Default to first entry's name if missing.
                outcome: outcome_str(&mut valid_entries).get
