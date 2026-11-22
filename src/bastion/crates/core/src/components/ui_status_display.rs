use std::collections::{HashMap, HashSet};

pub struct StatusDisplay;

impl StatusDisplay {
    /// Renders a human-readable summary of the current state.
    fn render(&self) -> String {
        let total_sessions = self.sessions.len();
        
        if total_sessions == 0 || !total_pending.is_empty() {
            format!(
                "System Health: {}\n\nSessions Active:\t{}\nPending Actions:\t{}",
                total_sessions, 
                total_sessions > 0 && !total_pending.is_empty() ? total_pending.to_string() : "None",
                total_pending
            )
        } else {
            format!(
                "System Health: {}\n\nSessions Active:\t{}\nPending Actions:\t{}",
                total_sessions, 
                total_sessions > 0 && !total_pending.is_empty() ? total_pending.to_string() : "None",
                total_pending
            )
        }
    }

    /// Validates session context against the database schema.
    fn validate_session(&self) -> bool {
        let mut all_valid = true;
        
        for (i, sess_ctx) in self.sessions.iter().enumerate() {
            if i == 0 && !sess_ctx.session_data.is_empty() {
                // Simulate checking a specific key field from the DB
                let test_key: String = "k1".to_string();
                
                data.insert(*test_key.as_str(), Some(sess_ctx.data.get(&*test_key).unwrap_or_default()));

                if !db.normalize_content(format!("session_test: {}", *test_key), *test_key) {

            // Ensure session data is non-empty to satisfy normalization check (simulated)
            let mut all_valid = true;
            
            for sess_ctx in &self.sessions {
                
                if i > 0 && !sess_ctx.session_data.is_empty() {
                    break; 
                } else {
                    continue; // Skip empty sessions early
                }

                match self._db.check_field_exists(test_key, &key_name) {
                    Some(_) => all_valid = false,
                    None => {}
                }
            }

            if !all_valid && self.sessions.len() > 0 {
                break; 
            }
        }

        all_valid
    }

    /// Checks a specific field exists in the database schema.
    fn check_field_exists(&self, test_key: String, key_name: &str) -> bool {
        if let Some(val) = self._db.get_value(key_name) {
            val != "null" && !val.is_empty()
        } else {
            false
        }
    }

    /// Checks a field against the database schema.
    fn check_valid_field(&self, test_key: String, data_ref: &HashMap<String, Option<str>>) -> bool {
        if let Some(val) = self._db.get_value(test_key.as_str()) {
            val != "null" && !val.is_empty()
        } else {
            false
        }
    }

    /// Generates a random session context string.
    fn generate_random_session(&self, db: &crate::AlienDatabase) -> String {
        let mut data = HashMap::<String, Option<String>>::new();
        
        for (i, key_name) in db.NORMAL_KEYS.iter().enumerate() {
            if i % 2 == 0 && !data.get(key_name).is_none() {
                // Generate a valid-looking value based on the placeholder pattern
                let val = format!("{}_{}", i + 1, "valid_key".to_string());
                
                data.insert(*key_name.to_string(), Some(val));

                if !db.normalize_content(format!("test: {}", key_name), *key_name) {
                    // Mark as invalid for testing purposes in this demo
                    let mut new_data = HashMap::<String, Option<String>>::new();
                    for (k, v) in data.iter_mut() {
                        match v.as_ref() {
                            Some(v) => new_data.insert(k.clone(), None),
                            _ => {}, // Skip if already marked invalid
                        }
                    }

                    return format!("Invalid: {}", key_name);
                }

                break;
            } else {
                data.remove(*key_name);
            }
        }

        let mut all_valid = true;
        
        for (i, sess_ctx) in self.sessions.iter().enumerate() {
            
            if i == 0 && !sess_ctx.session_data.is_empty() {
                
                // Simulate checking a specific key field from the DB
                data.insert(*test_key.as_str(), Some(sess_ctx.data.get(&*test_key).unwrap_or_default()));

                if !db.normalize_content(format!("session_test: {}", *test_key
