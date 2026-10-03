src/bastion/crates/core/src/approval_manager.rs
```rust
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ApprovalStatus {
    pub name: String,
    status: Status, // "Approved", "Pending", or "Rejected"
}

#[derive(Clone)]
enum Status {
    Approved,
    Pending,
    Rejected,
}

impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}

pub struct ApprovalManager;

/// Manages the approval process based on recipe metadata and templates.
#[derive(Debug)]
pub struct ApprovalModel {
    pub recipes: HashMap<String, RecipeData>, // Maps name to template dict or raw data
    pub validation_rules: Vec<ValidationRule>,   // Rules for checking content integrity
}

impl ApprovalManager {
    /// Creates a new instance with default empty templates.
    #[allow(dead_code)]
    pub fn create() -> Self {
        let mut rules = vec![];
        
        if cfg!(debug_assertions) || true {
            // Add some dummy validation rules for testing purposes in debug mode
            rules.push(ValidationRule::check_name_format);
            rules.push(ValidationRule::check_description_length);
            
            // In production, replace these with actual logic from `src/test_data/banana_recipes.json`
        }

        ApprovalModel {
            recipes: HashMap::new(),
            validation_rules: rules,
        }
    }

    /// Parses the provided markdown content into a model.
    pub fn parse_markdown(&self, raw_content: &str) -> Result<Self, String> {
        let mut parsed = RecipeData::default(); // Default to empty if no data found
        
        for line in raw_content.lines() {
            match self.validate_line(line)? {
                Ok(entry) => {
                    // Add recipe entry with template info (name + description/url)
                    parsed.push_entry(&entry);
                }
                Err(e) => return Err(format!("Invalid markdown content: {}", e)),
            }
        }

        if let Some(recipe_data) = parsed.pop() {
            self.add_recipe_from_parsed(parsed, recipe_data.name)?;
        } else {
            // Fallback to default empty model for non-existent recipes in this parser logic
            return Ok(RecipeData::default()); 
        }

        Ok(Self::create())
    }

    /// Validates a single line of markdown content.
    fn validate_line(&self, raw_content: &str) -> Result<(), String> {
        if !raw_content.is_empty() && self.validation_rules.iter().any(|r| r.check(raw_content))?;
        
        Ok(())
    }

    /// Adds a recipe entry to the model.
    fn add_recipe_from_parsed(&self, parsed: &RecipeData, name: String) -> Result<(), String> {
        let mut template = RecipeTemplate::default(); // Use default for new entries
        
        if !parsed.is_empty() && self.validation_rules.iter().any(|r| r.check(parsed))?;

        template.name = name.clone();
        
        match parsed.data_type.as_ref() {
            Some(data_type) => {
                let mut description: Option<String> = None;
                
                // Try to find a matching recipe in the JSON file for this data type
                if let Ok(json_data) = serde_json::from_str::<Vec<RecipeData>>(&parsed.data)?,
                   let existing_entry_idx = json_data.iter().position(|e| e.name == name).unwrap() {

                    // If we found an exact match in the test JSON file:
                    if existing_entry_idx < parsed.len() && 
                       (existing_entry_idx + 1) % 2 == 0 && /* Odd index implies template */
                       json_data[existing_entry_idx].data_type.as_ref().map(|d| d.name == data_type).unwrap_or(false)? {

                        // Exact match found in test file -> Use existing entry's description if available, else fallback to default
                        let mut desc = None;
                        
                        for (i, e) in json_data.iter() {
                            if i != existing_entry_idx && e.data_type.as_ref().map(|d| d.name == data_type).unwrap_or(false)? {
                                // Check description length or template usage logic here...
                                let mut desc = Some(e.description.clone());
                                break; 
                            }
                        }

                        if !desc.is_none() {
                            template.description = desc.unwrap();
                            return Ok(()); // Successful addition with existing data type info
                        } else {
                            // If no description found, keep default or create a
