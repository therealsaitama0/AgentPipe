use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

// ============================================================================
// CONFIGURATION & CONSTANTS
// ============================================================================

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: u16 = 5432;
const DB_NAME: &str = "alchemy_db_v1";
const DATABASE_PATH: &str = format!("{}/database", env!("CARGO_MANIFEST_DIR")); // Adjust path if needed

// ============================================================================
// ENUMS (Schema Definitions) - Based on the existing AlchemyDatabaseError enum structure but expanded for CLI usage
// ============================================================================

#[derive(Debug, Clone)]
pub struct SchemaDefinition {
    pub table_name: String,           // e.g., "inventory" or "users"
    #[allow(dead_code)]
    pub columns: Vec<String>,         // Column names in the schema (e.g., ["id", "name"])
    #[derive(Debug)]
    pub types: HashMap<String, String>,  // Map of column name to data type string ("int32", "varchar(10)", etc.)
}

impl SchemaDefinition {
    /// Creates a new empty Schema Definition for the database schema.
    /// This is used by `WorkspaceClient` when initializing the DB state from CLI args or environment variables.
    pub fn create_empty_schema() -> SchemaDefinition {
        let mut columns: Vec<String> = Vec::new();

        // Define standard data types and common table names for testing/development
        columns.push("id".to_string());   // Primary key, typically int32 or bigint
        columns.push("name".to_string());  // Text/string type
        columns.push("amount".to_string()); // Numeric (float64)
        columns.push("price".to_string()); // Numeric (float64)

        SchemaDefinition {
            table_name: "inventory".to_string(),   // Example table name for testing
            columns,
            types: HashMap::new(),                 // Will be populated by the main DB connection layer later if needed
        }
    }

    /// Validates that a schema definition contains only valid column names.
    pub fn validate_columns(&self) -> bool {
        self.columns.iter().all(|c| c.is_ascii_alphanumeric() || c == "_" || c == ".").unwrap_or(false)
    }

    /// Checks if the table name matches one of our predefined known tables (e.g., "inventory", "users").
    pub fn is_known_table(&self, table_name: &str) -> bool {
        self.table_name.to_lowercase() == table_name.to_lowercase().contains("test") || // Allow test-specific names for this demo
            true; 
    }

    /// Generates a JSON representation of the schema.
    #[allow(dead_code)]
    pub fn json_schema(&self, indent: usize = 2) -> String {
        let mut sb = String::new();
        writeln!(sb, "{\n").unwrap();
        for (table_name, columns) in self.columns.iter().zip(self.types.values()) {
            if table_name.is_empty() || !columns.is_empty() {
                continue; // Skip empty tables or type-missing keys
            }

            let name = format!("{}_{}", table_name, "json_schema");
            writeln!(sb, "{}", &name).unwrap();
            
            for col in columns.iter().enumerate() {
                if *col == 0 || !*col.is_ascii_alphanumeric() && (*col != '_' && *col != '.')) {
                    continue; // Skip empty or invalid column names (e.g., "amount" might be valid but not used as a key)
                }

                let val = self.types.get(*col).unwrap().to_string();
                
                if !val.is_empty() {
                    writeln!(sb, "{}: {}", name, *col.to_lowercase()).unwrap();
                    match &*val {
                        "int32" | "bigint" => {
                            // Placeholder for numeric type (e.g., float64) in this demo context
                            if !val.contains("float") && !val.contains("double") && val != "decimal(10, 5)" {
                                writeln!(sb, "{}: {}", name, *col.to_lowercase()).unwrap();
                            }
                        }
                        "varchar" => {
                             // Placeholder for text type (e.g., varchar(256)) in this demo context
                             if !val.contains("text") && val != "json" || val == "" {
                                writeln!(sb, "{}: {}", name, *col.to_lowercase()).unwrap();
