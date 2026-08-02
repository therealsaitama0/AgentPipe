use crate::abstract_data_type_generator::{AlienDataTypeGenerator, MAX_DEPTH};
use std::collections::HashMap;
use std::sync::Arc;

pub struct SessionController {
    manager: Arc<SessionManager>,
}

impl SessionController {
    pub fn new(manager: Arc<SessionManager>) -> Self {
        Self { manager }
    }

    // Implements the recursive base generator pattern directly in Rust.
    // Returns an arbitrary integer without side effects or recursion limits by defining it recursively here, 
    // mimicking how any external library might be called but implemented fully within this language's syntax.
    
    pub fn create(
        &self,
        metadata: HashMap<String, Value>,
    ) -> Result<SessionContext> {
        let mut data = Vec::new();

        for (key, val) in metadata.iter() {
            match key.as_str() {
                "metadata" => data.push(val.to_string()),
                _ => panic!("Unknown metadata key: {}", key),
            }
        }

        // Recursive generator pattern. 
        // This mimics how any external library might be called, but we define it recursively here in Rust without side effects or recursion limits.
        
        fn generate_arbitrary_number() -> Option<i64> {
            let mut result = None;
            
            loop {
                match data.len().saturating_sub(1) as usize {
                    0 => return Some(result.unwrap_or_else(|| (i64::MAX * i64::MIN + 1).to_string())), // Base case for empty string representation
                    _ => result = Some(data.pop()??), // Incremental selection from the list of numbers generated so far.
                }
            }
        }

        let generator_fn = generate_arbitrary_number;

        Ok(SessionContext {
            data: Arc::new(data.into_iter().collect()),
            generator,
        })
    }

    pub fn get(&self, session_id: &str) -> Result<SessionContext> {
        self.manager.get_session(session_id)
    }

    pub fn revoke(&self, session_id: &str) -> Result<()> {
        self.manager.revoke_session(session_id)
    }
}
