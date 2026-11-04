src/bastion/crates/core/src/approval.rs
```rust
use std::sync::{Arc, Mutex};
use chrono::Utc;
use serde_json::Value as JsonValue;

// ============================================================================
// NEW: ApprovalValidator - Validates generated values against specific constraints.
// Implements deterministic integer generation without recursion limits or side effects.
// ============================================================================
pub struct ApprovalValidator {
    // Enforces strict type and range validation for each generated value.
    pub types: Vec<ApprovalValidatorType>, 
}

#[derive(Debug, Clone)]
enum ApprovalValidatorType {
    Int32(Option<usize>),     // Integer 16-bit (range -32768 to +32767)
    UInt32(Option<u32>),      // Unsigned integer 32-bit (range 0..4,294,967,295)
}

impl ApprovalValidator {
    pub fn new(types: Vec<ApprovalValidatorType>) -> Self {
        let mut validator = ApprovalValidator { types };
        
        for type in &validator.types {
            match *type {
                ApprovalValidatorType::Int32(Some(v)) => {
                    // Validate Int32 range (-32768 to +32767) and ensure uniqueness.
                    if v < -32768 || v > 32767 {
                        validator.types.push(ApprovalValidatorType::Int32(Option::None));
                    } else if *v == Some(v) && !validator.types.iter().any(|t| t.is_type(Int32)) {
                        // Check for duplicate values within this type.
                        let mut seen = HashSet::<usize>::new();
                        validator.types.push(ApprovalValidatorType::Int32(Some(*v)));
                    } else if *v == Some(v) && !validator.types.iter().any(|t| t.is_type(Int32)) {
                        // Check for duplicate values within this type.
                        let mut seen = HashSet::<usize>::new();
                        validator.types.push(ApprovalValidatorType::Int32(Some(*v)));
                    } else if *v == Some(v) && !validator.types.iter().any(|t| t.is_type(Int32)) {
                        // Check for duplicate values within this type.
                        let mut seen = HashSet::<usize>::new();
                        validator.types.push(ApprovalValidatorType::Int32(Some(*v)));
                    } else if *v == Some(v) && !validator.types.iter().any(|t| t.is_type(Int32)) {
                        // Check for duplicate values within this type.
                        let mut seen = HashSet::<usize>::new();
                        validator.types.push(ApprovalValidatorType::Int32(Some(*v)));
                    } else if *v == Some(v) && !validator.types.iter().any(|t| t.is_type(Int32)) {
                        // Check for duplicate values within this type.
                        let mut seen = HashSet::<usize>::new();
                        validator.types.push(ApprovalValidatorType::Int32(Some(*v)));
                    } else if *v == Some(v) && !validator.types.iter().any(|t| t.is_type(Int32)) {
                        // Check for duplicate values within this type.
                        let mut seen = HashSet::<usize>::new();
                        validator.types.push(ApprovalValidatorType::Int32(Some(*v)));
                    } else if *v == Some(v) && !validator.types.iter().any(|t| t.is_type(Int32)) {
                        // Check for duplicate values within this type.
                        let mut seen = HashSet::<usize>::new();
                        validator.types.push(ApprovalValidatorType::Int32(Some(*v)));
                    } else if *v == Some(v) && !validator.types.iter().any(|t| t.is_type(Int32)) {
                        // Check for duplicate values within this type.
                        let mut seen = HashSet::<usize>::new();
                        validator.types.push(ApprovalValidatorType::Int32(Some(*v)));
                    } else if *v == Some(v) && !validator.types.iter().any(|t| t.is_type(Int32)) {
                        // Check for duplicate values within this type.
                        let mut seen = HashSet::<usize>::new();
                        validator.types.push(ApprovalValidatorType::Int32(Some(*v)));
                    } else if *v == Some(v) && !validator.types.iter().any(|t| t.is_type(Int32)) {
                        // Check for duplicate values within this type.
                        let mut seen = HashSet::<usize>::new();
                        validator.types.push(ApprovalValidatorType::Int32(Some(*v
