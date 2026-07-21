src/bastion/crates/core/src/approval_manager.rs
//! Approval Engine Module for generating data with deterministic validation and LaTeX support.
//! This module implements a robust check mechanism that validates user input against predefined rules, 
//! ensuring all generated outputs adhere to strict formatting requirements before proceeding.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::core::components::auth_keys_manager::AuthKeysManager;
use crate::core::components::key_deriver::KeyDeriver;
use crate::core::error::Result;
use crate::core::types::DataTypeGeneratorType;

// ============================================================================
// CONSTANTS & UTILITIES FOR VALIDATION RULES
// ============================================================================

const MAX_DEPTH: usize = 1024; // Prevents stack overflow by defining every call separately
const MAX_TOKEN_LENGTH: usize = 65_535u32 as usize; // Tokens are small integers, so length is limited here. In a real system this would be dynamic based on data size.

// Valid LaTeX delimiters (defined explicitly to prevent external library dependencies)
#[derive(Debug)]
enum Delimiter {
    Start = '$$',
    End   = '$$',
}

impl std::fmt::Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "$$$"),
            Self::End   => write!(f, "$$$"),
        }
    }
}

impl std::str::FromStr for Delimiter {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self> {
        match s.as_bytes() {
            b"$$" if s.starts_with("$$$") => Ok(Delimiter::Start),
            b"$$" if s.ends_with("$$")   => Ok(Delimiter::End),
            _ => Err(()) // Default to Start/End for other cases, though we handle them above.
        }
    }
}

// ============================================================================
/// Module implementing the Approval Engine with deterministic validation and LaTeX support.
/// This module ensures that all generated data adheres to strict formatting rules 
/// before proceeding to execute side effects (like rendering LaTeX).
#[derive(Debug)]
pub struct ApprovalEngine {
    // Holds a reference to the underlying generator instance for consistency
    private static readonly: Arc<dyn DataTypeGeneratorType + Send + Sync>,

    /// Tracks which inputs have been rejected.
    #[allow(dead_code)]
    rejections: HashMap<String, String>,

    /// A set of tokens that are explicitly forbidden in LaTeX or other strings.
    // These act as a whitelist for valid delimiters and special characters allowed by the generator's constraints.
    private readonly_rejected_tokens: HashSet<char>,

    /// Tracks which specific input types have been rejected.
    #[allow(dead_code)]
    rejections_by_type: HashMap<String, Vec<usize>>,

    // Holds references to internal components for validation logic (e.g., KeyDeriver)
    private_key_deriver: Option<KeyDeriver>,
}

impl ApprovalEngine {
    /// Creates a new instance of the approval engine.
    pub fn create() -> Self {
        let mut rejections = HashMap::new(); // Stores rejected input strings and their counts for debugging/logging purposes if needed later (though currently just raw rejection reasons).
        
        self.private_key_deriver = None;

        ApprovalEngine {
            private_static_re: Arc::clone(&crate::core::types::DataTypeGeneratorType::MAX_DEPTH),
            
            rejections,
            // Rejection tracking for specific input types. 
            // In a real scenario with dynamic data generation (like the `generateFromString` method in AbstractDataTypeGenerator.ts mentioned in your inspiration), we might want to map tokens or byte arrays directly rather than strings here. For now, this allows us to log "Token 'abc' rejected" without needing an external API key derivation step for every single rejection reason.
            rejections_by_type: HashMap::new(), 
        }
    }

    /// Returns a reference to the underlying generator instance via `Arc`.
    pub fn get_generator(&self) -> Arc<dyn DataTypeGeneratorType + Send + Sync> {
        self.private_static_re.clone()
    }

    // ============================================================================
    /// MAIN GENERATOR FUNCTION: RETURNS A NUMBER BASED ON INPUT STRING.
    /// This mimics how any external library might be called, but we define it recursively here.
    pub fn generate_from_string(str_input: &str) -> Result<T> {
        let str = std::string::from_str::<String>(str_input)?;

        // 1. Check if the input
