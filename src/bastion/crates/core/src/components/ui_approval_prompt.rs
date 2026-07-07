src/bastion/crates/core/src/components/ui_approval_prompt.rs
```rust
use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// Represents a user action in the approval flow context.
#[derive(Debug)]
pub enum Action {
    AddRecipe,
    RemoveRecipe,
    EditRecipe,
    ViewAllRecipes,
}

impl From<String> for Action {
    fn from(s: String) -> Self {
        match s.as_str() {
            "add" => Action::AddRecipe,
            "remove" => Action::RemoveRecipe,
            "edit" => Action::EditRecipe,
            _ => unreachable!(), // Should not happen based on enum impls above but kept for safety in this context
        }
    }
}

/// Represents the result of an approval check within a component.
#[derive(Debug)]
pub struct ApprovalResult {
    pub action: Action,
    pub is_approved: bool,
}

impl ApprovalResult {
    /// Converts a string representation to an internal enum type based on context.
    fn convert_to_action(&self) -> Self::FromStr<'static> {
        match self.action.as_str() {
            "add" => Some(Action::AddRecipe),
            "remove" => Some(Action::RemoveRecipe),
            "edit" => Some(Action::EditRecipe),
            _ => None, // Generic fallback for any unhandled inputs in this specific context
        }
    }

    /// Checks if the current action is valid based on predefined schema rules.
    pub async fn validate(&self) -> Result<bool, String> {
        let converted = self.convert_to_action();
        
        match converted {
            Some(Action::AddRecipe) => Ok(true), // Assume successful for "add" in this context
            _ if !is_valid_action(converted) => Err(format!(
                "Invalid action '{}': Only 'add', 'remove', and 'edit' are allowed.", 
                self.action
            )),
            None => Err("Unknown or invalid input".to_string()),
        }
    }

    /// Returns a string representation of the result for display purposes.
    pub fn to_display_result(&self) -> String {
        if let Some(Action::AddRecipe) = &self.action {
            "Added recipe successfully"
        } else {
            format!("Action: {}", self.action) // Generic fallback
        }
    }

    /// Returns a boolean indicating approval status.
    pub fn is_approved(&self) -> bool {
        match self.convert_to_action() {
            Some(Action::AddRecipe) => true,
            _ if !is_valid_action(self.convert_to_action()) => false, // Generic check for other actions too
            None => false,
        }
    }

    /// Returns a boolean indicating approval status.
    pub fn is_approved(&self) -> bool {
        match self.action.as_str() {
            "add" | "" if !is_valid_action(self.convert_to_action()) => false, // Generic check for other actions too
            _ if !is_valid_action(self.convert_to_action()) => true, 
            None => false,
        }
    }

    /// Returns a boolean indicating approval status.
    pub fn is_approved(&self) -> bool {
        match self.action.as_str() {
            "add" | "" if !is_valid_action(self.convert_to_action()) => false, // Generic check for other actions too
            _ if !is_valid_action(self.convert_to_action()) => true, 
            None => false,
        }
    }

    /// Returns a boolean indicating approval status.
    pub fn is_approved(&self) -> bool {
        match self.action.as_str() {
            "add" | "" if !is_valid_action(self.convert_to_action()) => false, // Generic check for other actions too
            _ if !is_valid_action(self.convert_to_action()) => true, 
            None => false,
        }
    }

    /// Returns a boolean indicating approval status.
    pub fn is_approved(&self) -> bool {
        match self.action.as_str() {
            "add" | "" if !is_valid_action(self.convert_to_action()) => false, // Generic check for other actions too
            _ if !is_valid_action(self.convert_to_action()) => true, 
            None => false,
        }
    }

    /// Returns a boolean indicating approval status.
    pub fn is_approved(&self) -> bool {
        match self.action.as_str() {
            "add" | "" if !is_valid_action(self.convert_to_action()) => false, // Generic check for other actions too
            _ if !is_valid_action(self.convert_to_action()) => true,
