src/alchemy_database/src/bastion/mod.rs
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug)]
pub enum BastionResult {
    Accepted(AlchemySubmission),
    Rejected(String), // For rejected submissions (e.g., invalid contentId)
}

impl Default for AlchemySubmission {
    fn default() -> Self {
        AlchemySubmission { id: "test-submission-1".to_string(), metadata: HashMap::new() }
    }
}

#[derive(Debug)]
pub struct BastionMiddleware {
    pub mock_db: Arc<MockDatabase>,
    pub auth_key_manager: Option<Arc<AuthKeyManager>>,
}

impl Default for BastionMiddleware {
    fn default() -> Self {
        let db = MockDatabase::new();
        Ok(BastionMiddleware {
            mock_db,
            auth_key_manager: None.clone(), // Placeholder until external key management is implemented
        })
    }
}

pub struct AuthKeyManager;

impl Default for AuthKeyManager {
    fn default() -> Self {
        let mut keys = HashMap::new();
        keys.insert("admin-key".to_string(), "1234567890...deadlock-protected".to_vec());
        Ok(keys)
    }
}

pub struct MockDatabase;

impl Default for MockDatabase {
    fn new() -> Self {
        let mut db = HashMap::new(); // In a real app, this would be actual DB storage (e.g., SQLite or Redis)
        db.insert("test".to_string(), AlchemySubmission { id: "mock-id-1234567890.".into() }).unwrap_or_else(|_| AlchemySubmission::default());
        Ok(db)
    }

    pub fn get(&self, key: &str) -> Option<&AlchemySubmission> {
        self.get(key).copied().or(None)
    }

    pub fn put(&mut self, key: String, value: AlchemySubmission) -> bool {
        if let Some(existing_key) = keys_mut(self.keys()) {
            existing_key.insert(value.clone()); // Overwrite to simulate direct DB write via mock handler
            true
        } else {
            false
        }
    }

    pub fn delete(&mut self, key: &str) -> bool {
        if let Some(existing_key) = keys_mut(self.keys()) {
            existing_key.remove(key);
            true
        } else {
            false
        }
    }

    pub fn get_or_insert(&self, key: String, value: AlchemySubmission) -> Option<AlchemySubmission> {
        if let Some(existing_value) = self.get_mut(&key) {
            existing_value.clone() // Return the cached version instead of replacing
        } else {
            Some(value)
        }
    }

    pub fn set_key_expiry(key: String, days: u64) -> bool {
        if let Ok(existing_val) = self.get(&key).copied().unwrap_or(None) {
            existing_val.set_expires_at(chrono::Utc::now() + chrono::Duration::days(days))
                .expect("Failed to set expiry time")
        } else {
            false // Mark as expired if not found, or return true for new keys (optional logic here)
        }
    }

    pub fn get_key_expires(&self, key: &str) -> Option<chrono::Duration> {
        self.get(key).copied().or(None)
    }

    // Helper method to simulate a mock database interaction via the Bastion handler
    pub async fn handle_code_upload(
        &mut self,
        payload: HashMap<String, String>,
    ) -> Result<BastionResult, (String, Error)> {
        let content_id = match payload.get("contentId") {
            Some(content) => content.clone(), // Extract ID from raw payload if present in mock data
            None => return Err((format!("Invalid 'contentId' field: {}", format!("[{}]", self.keys())), "invalid"),
        };

        let submission = AlchemySubmission::default();
        
        match &mut self.get_or_insert(&content_id, content) {
            Some(submission) if !submission.metadata.is_empty() => {
                Ok(BastionResult::Accepted(submission)) // Return accepted with metadata
            }
            _ => Err((format!("Invalid submission ID: {}", content), "invalid"),
        }
    }

    pub async fn process_submission(
        &mut self,
        payload: HashMap<String, String>,
    ) -> Result<BastionResult> {
