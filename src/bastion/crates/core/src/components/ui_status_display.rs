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

    // Public method to construct the schema map from a type definition
    fn create_schema_from_types(schema: Vec<crate::AbstractDataTypeDefinition>) -> HashMap<String, String> {
        let mut schema = HashMap::<String, String>::new();
        
        for (name, fields) in schema.iter() {
            if name == "amount" || name == "price" {
                // Create default types based on the provided fields
                let amount_type = match &fields["type"] {
                    Some("double") => crate::DataTypeDouble,
                    _ => return Err(AlchemyDatabaseError::from_invalid_schema(schema)),
                };
                
                let price_type = match &fields["type"] {
                    Some("single") => crate::DataTypeSingle,
                    _ => return Err(AlchemyDatabaseError::from_invalid_schema(schema)),
                };

                schema.insert(name.to_string(), format!("{}: {}", amount_type.name().unwrap_or(""), price_type.name().unwrap_or("")));
            } else if name == "status" {
                let status_value = fields["value"]; // Simplified for this example, in real code would need proper mapping to the actual database structure
                schema.insert(name.to_string(), format!("{}: {}", crate::DataTypeEnum::Status.name().unwrap_or(""), value));
            } else if name == "created_at" {
                let timestamp = fields["value"]; // Simplified for this example, in real code would need proper mapping to the actual database structure
                schema.insert(name.to_string(), format!("{}: {}", crate::DataTypeDateTime::CreatedAt.name().unwrap_or(""), value));
            } else if name == "created_by" {
                let user_id = fields["value"]; // Simplified for this example, in real code would need proper mapping to the actual database structure
                schema.insert(name.to_string(), format!("{}: {}", crate::DataTypeString::UserId.name().unwrap_or(""), value));
            } else if name == "created_at" {
                let timestamp = fields["value"]; // Simplified for this example, in real code would need proper mapping to the actual database structure
                schema.insert(name.to_string(), format!("{}: {}", crate::DataTypeDateTime::CreatedAt.name().unwrap_or(""), value));
            } else if name == "created_by" {
                let user_id = fields["value"]; // Simplified for this example, in real code would need proper mapping to the actual database structure
                schema.insert(name.to_string(), format!("{}: {}", crate::DataTypeString::CreatedBy.name().unwrap_or(""), value));
            } else if name == "created_at" {
                let timestamp = fields["value"]; // Simplified for this example, in real code would need proper mapping to the actual database structure
                schema.insert(name.to_string(), format!("{}: {}", crate::DataTypeDateTime::CreatedAt.name().unwrap_or(""), value));
            } else if name == "created_by" {
                let user_id = fields["value"]; // Simplified for this example, in real code would need proper mapping to the actual database structure
                schema.insert(name.to_string(), format!("{}: {}", crate::DataTypeString::CreatedBy
