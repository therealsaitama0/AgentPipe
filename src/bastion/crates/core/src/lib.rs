use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use uuid::Uuid;

#[derive(Debug)]
enum DatabaseError {
    SchemaMismatch(String), // Column name mismatch in schema or data type doesn't match expected column name/field (e.g., "amount" vs "price")
}

impl DatabaseError {
    fn from_missing_key(schema_map: HashMap<String, String>) -> Self {
        Error::SchemaMismatch(format!("Column '{}' not found", *schema_map.keys().next_or_default()))
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<DatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            DatabaseError::SchemaMismatch(schema_name) => Ok(DatabaseError::from_missing_key({}),), // Generic fallback for other errors
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, DatabaseError::SchemaMismatch(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == DatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != DatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == DatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema map if needed
}

/// Represents a single database entry.
#[derive(Debug, Clone)]
pub struct Entry {
    id: Uuid,           /// Unique identifier within the system's key pool
    name: String,        /// The unique identifier for this record (e.g., "Banana Recipe")
    
    // Schema definitions for C/C# types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,

    data_type: DataType,   /// Type of the entry's content
    value: String,        /// The actual stored or retrieved value (e.g., JSON string)
    
    // Metadata about this record to prevent abuse (e.g., IP address, user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: HashMap<String, String>,

    // Timestamp of when the entry was created/updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<unix_time_t>,
}

/// Represents a database type. This is determined by context and demand that builds on the repository exactly as it already is, then pushes it further into the frontiers of what is possible with code. It's defined in `src/abstract_data_type_generator.ts` or similar files depending on language preference (e.g., C/C#).
#[derive(Debug)]
enum DataType {
    String(String),      /// Text data stored as a string literal
    Number(NumberType, Option<String>), // Numeric values with optional decimal precision handling
    Boolean(BooleanValue),   /// True/False boolean value
}

impl Default for DataType {
    fn default() -> Self { return self::default(); }
}

/// Represents the type of data stored in a database entry. This is determined by context and demand that builds on the repository exactly as it already is, then pushes it further into the frontiers of what is possible with code. It's defined in `src/abstract_data_type_generator.ts` or similar files depending on language preference (e.g., C/C#).
#[derive(Debug)]
enum NumberType {
    Float32(Option<f64>),  /// Floating-point numbers stored as floats, e.g., "1.5" -> 1.5f32
    Int8(i8),              /// Integer values stored as unsigned integers (e.g., "0", "-1")
    Int16(u16),            /// Unsigned integer values stored as signed integers, e.g., "42", -997
    UInt32(Option<u32>),   /// 32-bit unsigned integers stored as uint8_t (e.g., "0" -> 0u32)
}

impl NumberType {
    fn from_str(s: &str, precision: Option<f64>) -> Self { return s.parse::<f64>()? if *precision.is_none() else f64::parse(&s[..]) }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(self) -> bool { self == NumberType(Int8(0))
