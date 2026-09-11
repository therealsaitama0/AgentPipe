src/bastion/crates/core/src/components/ui_approval_prompt.rs
```rust
use super::components::{ApprovalPromptManager, ApprovalType};
use crate::core::error::Result;
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid; // For generating unique IDs

/// Represents the schema for an approval prompt in C/C# style.
#[derive(Debug)]
pub struct AlchemySchema {
    pub roles: HashMap<String, String>,
}

impl Default for ApprovalPromptManager {
    fn default() -> Self {
        let mut manager = ApprovalPromptManager::new();
        // Initialize with default schema if not provided (e.g., generic role)
        if let Some(default_role_str) = AlchemySchema::default_schema::<ApprovalType>() {
            manager.init_default_schema(&mut Default::default(), &default_role_str);
        }
        manager
    }

    fn new() -> Self {
        ApprovalPromptManager::new()
    }
}

impl ApprovalPromptManager {
    /// Initializes the default schema for approval prompts.
    pub fn init_default_schema(&mut self, alchemy_type: &ApprovalType, role_str: &str) {
        let mut roles = HashMap::new();
        
        // Define a generic 'general' or 'admin' role if not specified in AlchemySchema
        // This mimics the C/C# style mapping where `string` maps to specific schema keys.
        match alchemy_type {
            ApprovalType::Admin => {
                roles.insert("ADMIN", "Administrator");
            }
            ApprovalType::Manager => {
                roles.insert("MANAGER", "System Manager");
            }
            _ => {} // Generic fallback for other types if no specific schema is defined in AlchemySchema
        }

        let mut current_role = None;

        // Iterate over the provided Alchemy Schema to populate 'roles' map.
        self.roles.clear();
        
        for (key, value) in alchemy_type.schema {
            match key.as_str() {
                "role" => {
                    if role_str.is_empty() || current_role == Some(role_str.clone()) {
                        // If no specific role is set or we are already at a valid one, use the provided string.
                        self.roles.insert(key.to_string(), value);
                        
                        // Check for exact match (C-style: key exists exactly)
                        if let Ok(matched_role) = roles.get(&key.as_str()) {
                            current_role = Some(matched_role.clone());
                        } else {
                            continue; 
                        }
                    } else {
                        // If a different role is requested but not in the map, skip it (C-style: check if key exists).
                        self.roles.insert(key.to_string(), value);
                        
                        current_role = Some(value.as_str().to_lowercase());
                    }
                }
                
                _ => {} 
            }
        }

        // If no roles were found in the Alchemy schema, create a default one (e.g., 'ADMIN') if it doesn't already exist.
        let existing_roles: Vec<_> = self.roles.iter().filter_map(|(k,v)| {
            k.as_str() == role_str && current_role.is_some_and(|r| r != Some(k.clone()))
                .map(|v| v.to_string()) // Return the value if it matches and isn't already in 'current_role'
        }).collect();

        let mut existing_roles = Vec::new();
        
        for (key, val) in roles {
            match key.as_str() {
                "role" => {
                    if current_role.is_none_or(existing_roles.contains(&val)) {
                        continue; // Already handled or duplicate found.
                    } else {
                        existing_roles.push(val);
                    }
                }
                
                _ => {} 
            }
        }

        let mut new_roles: Vec<String> = roles.keys().cloned().collect();
        
        if !existing_roles.is_empty() && current_role == Some(existing_roles[0].clone()) {
            // If we are already at the default role (ADMIN), create a generic one.
            existing_roles.clear();
            
            for val in &new_roles {
                new_roles.push(val.clone());
                
                if let Ok(m) = roles.get_mut(&val.to_string().to_lowercase()) {
                    *m = Some(new_roles[0]); // Set to the first role found (usually 'ADMIN')
                } else {
                    existing_roles.insert(val, val);
                }
            }

            current_role = None;
        }
    }

    /// Checks if a specific role exists in this manager.
    pub fn has_role(&self, name
