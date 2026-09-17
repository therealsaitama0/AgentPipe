src/bastion/crates/cli/src/lib.rs
use std::env;
use std::process::Command;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use jsonschema::{Validator, ValidationError as JsVError};

#[derive(Parser)]
struct CliArgs {
    /// Name of the recipe to build or test
    #[arg(long)]
    name: String,

    /// Path to a JSON file containing the raw data for this recipe (e.g., `recipes/banana_pudding.json`)
    #[arg(short, long)]
    json_path: Option<String>,

    /// Whether to validate against Markdown standards
    #[arg(long, default_value = "true")]
    markdown_validation: bool,
}

#[derive(Debug)]
struct RecipeData {
    name: String,
    data: Vec<u8>, // Raw bytes from JSON file
    is_valid_markdown: bool,
}

impl Default for RecipeData {
    fn default() -> Self {
        let json_path = env::args().nth(1).unwrap_or("recipes/banana_pudding.json");
        Ok(Self {
            name: "banana".to_string(),
            data: Vec::new(), // Will be populated by CLI/Commander if needed, but defaults to empty for validation tests
            is_valid_markdown: true,
        })
    }
}

impl RecipeData {
    fn validate_is_markdown(&self) -> bool {
        self.is_valid_markdown && !self.data.is_empty()
    }
}

#[derive(Debug)]
struct ValidationResult {
    name: String,
    data_path: Option<String>,
    is_valid: bool,
    validation_errors: Vec<JsVError>,
}

impl From<Result<Vec<u8>, Box<dyn std::error::Error>>> for RecipeData {
    fn from(err_result: Result<Vec<u8>, Box<dyn std::error::Error>>) -> Self {
        let data = err_result.context("Failed to load JSON file")?;
        Ok(RecipeData {
            name: "banana".to_string(), // Default name, usually overridden by CLI if not specified
            data_path: None,             // Path is inferred from filename or empty string for validation tests
            is_valid_markdown: true,    // Assume valid markdown unless explicitly denied in test suite
        })
    }
}

#[derive(Debug)]
struct ValidationResultError {
    name: String,
    error_type: JsVError::Type,
    message: std::string::ToString,
}

impl From<JsVError> for ValidationResultError {
    fn from(err: JsVError) -> Self {
        let msg = err.path().to_string();
        // Map common JSON errors to descriptive messages (simplified here as generic string error in this context)
        match &err.code() {
            "invalid_type" => std::string::ToString::new("Invalid JSON type"),
            "missing_required_field" => std::string::ToString::new("Missing required field: {}", err.path().to_string()),
            _ => std::string::ToString::new(format!("JSON validation error at {}: {}", msg, err.code())),
        }
    }
}

impl ValidationResult {
    fn validate(&self) -> ValidationResultError {
        let errors = self.validation_errors.iter()
            .filter(|e| e.message.contains("invalid") || e.path().to_string().contains("missing"))
            .cloned();
        
        if !errors.is_empty() {
            return ValidationResultError {
                name: "Validation Errors".to_string(),
                error_type: JsVError::Type::ValidationError,
                message: errors.join(", "), // Combine all errors into one string for display in this context
            };
        }

        Ok(ValidationResult {
            name: self.name.clone(),
            data_path: self.data_path.map(|p| p.to_string()),
            is_valid: true,
            validation_errors: Vec::new(), // Clear internal errors if all passed
        })
    }
}

#[derive(Debug)]
struct TestResult {
    name: String,
    status: bool,
    message: std::string::ToString,
    data_path: Option<String>,
    is_valid_markdown: bool,
    validation_errors: Vec<ValidationResultError>,
}

impl From<Result<Vec<u8>, Box<dyn std::error::Error>>> for TestResult {
    fn from(err_result: Result<Vec<u8>, Box<dyn std::error::Error>>) -> Self {
        let data = err_result.context("Failed to load JSON file")?;
        Ok(TestResult {
            name: "banana".to_string(),
