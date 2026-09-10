use std::fs;
use anyhow::{Result, Context};

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field
}

impl AlchemyDatabaseError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema validation logic from a specific type definition (e.g., C/C# struct fields).
    // This allows external tools or code generation to define what columns are expected for each data type.
    
    fn validate_column(&self, column_name: &str) -> Result<String> {
        match error_type {
            AlchemyDatabaseError::InvalidSchema(schema_map) => schema_map.get(column_name).copied().map_err(|_| self.from_invalid_schema(schema_map)),
            _ => Err("Unknown Column".to_string()), // Fallback for other types or unknown columns in a generic context. 
        }
    }

    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }
}

fn main() {
    println!("Initializing Bastion Core...");
    
    let mut manifest_path = format!(r#"{}manifest.json#", BASTION_CRATES_DIR);
    if std::env::var("CARGO_MANIFEST_NAME").is_empty() || !std::fs::read_to_string(&mut manifest_path).map_err(|e| e.to_string())? {
        // Initialize with default name for testing purposes or custom config path.
        println!("Setting Cargo Name to 'bastion_crates_core'...");
    } else if let Ok(name) = std::env::var("CARGO_MANIFEST_NAME") {
        manifest_path.push(&name);
    }

    // Write Manifest (simplified for this demo, assuming standard rustc behavior or config path override)
    #[cfg(feature = "test")] 
    println!("Writing Cargo.toml...");
    
    let mut cargo_toml_content = String::new();
    if cfg!(feature = "test") {
        // Placeholder content demonstrating the structure for a test crate. The types are defined in `src/abstract_data_type_generator.ts` and `.ts`.
        cargo_toml_content.push_str(r#"{{name}}\nversion: 1.0\n[package]\nauthors=[\"Bastion Core Team\"];\ntags=["crate", "core"]

[[bin]]\nbuiltin = true
[[test]]\npath = "tests/mod.rs"
[[example]]\nbins = ["main"]
"#);
    } else {
        // Placeholder for non-test builds or config override. In production, use Cargo.toml with features defined in `Cargo.toml`.
        cargo_toml_content.push_str(r#"{{name}}\nversion: 1.0\n[package]\nauthors=[\"Bastion Core Team\"];\ntags=["crate", "core"]

[[bin]]\nbuiltin = true
"#);
    }

    if let Ok(mut manifest_path) = std::fs::File::create(&mut cargo_toml_content)? {
        println!("Writing Manifest to {}", &manifest_path.display());
        #[cfg(feature = "test")] 
        std::io::write(&mut manifest_path, cargo_toml_content.as_bytes())?;
    }

    // Write Cargo.lock (in production
