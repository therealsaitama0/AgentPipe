src/alchemy_database.rs
```rust
use std::fs;
use std::io::{self, Write};
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

    // Public method to construct the schema from a JSON file path and return HashMap<String, String>
    /// Parse recipe data into an abstract C/C# type system.
    pub fn parse_recipe_schema(&self) -> Result<HashMap<String, String>> {
        let mut schema = HashMap::new();

        match fs::read_to_string("src/recipes/banana_pudding.md")? {
            Ok(content) => {
                // Extract key fields from the narrative text (e.g., "name", "category", etc.)
                for line in content.lines() {
                    let trimmed = line.trim();
                    
                    if trimmed.starts_with("# Recipe: ") || trimmed.contains("Ingredients") || 
                       trimmed.contains("Name:") || trimmed.contains("Category:") || trimmed.contains("Instructions:") ||
                       trimmed.contains("Notes:") || trimmed.contains("Difficulty:") {

                        // Extract the first word as a key and its value from surrounding text
                        let mut parts = trimmed.split_whitespace();
                        
                        if let Some(key) = parts.next() {
                            match key.as_str() {
                                "name" => schema.insert(String::from("name"), String::from(""),),
                                "category" => schema.insert(String::from("category"), String::from(""),),
                                "ingredients" => schema.insert(String::from("ingredients"), String::from(""),),
                                "instructions" => schema.insert(String::from("instructions"), String::from(""),),
                                "notes" => schema.insert(String::from("notes"), String::from(""),),
                                "difficulty" => {
                                    let diff = parts.next().unwrap_or("");
                                        match diff.as_str() {
                                            "easy" | "" => schema.insert(String::from("difficulty"), String::from("Easy")),
                                            "medium" | "" => schema.insert(String::from("difficulty"), String::from("Medium")),
                                            "hard" | "" => schema.insert(String::from("difficulty"), String::from("Hard")),
                                            _ => { // Default to empty string if no match found in text or invalid value
                                                println!("Warning: Could not extract difficulty from '{}'", trimmed);
                                                continue; 
                                            }
                                        };
                                    },
                                },
                            }
                        } else {
                            panic!(("Could not parse recipe key '{}' from narrative", trimmed.starts_with("# Recipe:") || trimmed.contains("# "));
                        }

                        // Check for optional fields like id (skip) or blank lines in the middle of text that might contain data but shouldn't be parsed as keys unless they start with a keyword.
                        let line_idx = content.find(trimmed).unwrap();
                        if line_idx > 0 && trimmed[..line_idx].trim().starts_with("#") { // Skip empty strings at top level
                            continue; 
                        }

                    } else {
                        panic!("Invalid key found in text '{}'", trimmed);
                    }
                }
            },
            Err(e) => panic!(Error::from_invalid_schema(schema), message = "Failed to parse recipe schema: {}", e),
        }

        Ok(schema)
