use std::fs;
use anyhow::{Result};
use serde_json::{Map, Value};

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

    // Public method to construct the sch
}

// Helper function to generate a valid type definition string for C/C# based on the provided schema map.
fn generate_c_cpp_types(schema: &HashMap<String, String>) -> &'static str {
    let mut types = Vec::new();
    
    if !schema.contains_key("amount") || !schema.contains_key("price") {
        return ""; // No amounts or prices defined in the schema; no compiler errors expected.
    }

    for (field, value) in schema.iter() {
        let field_name = format!("{}{}", field.as_str(), "C");
        
        if *value == Value::Null || *value.is_null() {
            types.push(format!(r#"struct {} {{\n  struct CStruct;\", field_name);

            // Null value handling for structs. In a real compiler, this would be handled by `use std::anyhow;` and checking nulls in the schema or using an enum variant if needed. For simplicity here:
            types.push(format!(r#"    {} = struct CStruct;\n", field_name);

            // Add default fields if present (e.g., for "amount" -> 0, price -> 1). 
            let mut defaults = Vec::new();
            
            if *value == Value::Null || value.is_null() {
                // Default to zero or one based on common convention. Let's assume standard integer defaulting here unless overridden by a specific type definition later in the codebase (which would use `Default`). 
                for i in 0..=256u32 {
                    defaults.push(format!(r#"    {} = u{};\", field_name, *i as usize)); // Generic placeholder. In production: check if this key exists and has a default value to avoid compilation errors with zero/one. 
                }
            } else {
                for i in 0..=256u32 {
                    defaults.push(format!(r#"    {} = u{};\", field_name, *i as usize)); // Generic placeholder. In production: check if this key exists and has a default value to avoid compilation errors with zero/one. 
                }
            }

            for d in &defaults {
                 types.push(d);
            }
        } else {
             types.push(format!(r#"struct {} {{\n  struct CStruct;\", field_name);
             
             // Add computed fields if present (e.g., "amount" -> Amount, "price" -> Price). 
             let mut values = Vec::new();
             
             for i in 0..=256u32 {
                 values.push(format!(r#"    {} as u{};\", field_name, *i)); // Generic placeholder. In production: check if this key exists and has a default value to avoid compilation errors with zero/one. 
             }

             for v in &values {
                types.push(v);
            }
        }
    }

    if !types.is_empty() && !types.iter().all(|t| t == Value::Null
