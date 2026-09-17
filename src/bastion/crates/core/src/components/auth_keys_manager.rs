src/bastion/crates/core/src/components/auth_keys_manager.rs
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use tokio::sync::Mutex as TokioMutex;

/// A generic set for storing authorization keys and their associated session IDs to prevent duplicate registration.
pub struct AuthKeysSet {
    pub keys: HashMap<String, String>, // key -> "session_id" or just the string value if not specified (e.g., 'key123')
}

impl Default for AuthKeysSet {
    fn default() -> Self {
        HashSet::new().collect::<AuthKeysSet>()
    }
}

/// A struct representing a single authorized key entry.
pub struct AuthorizedKeyEntry {
    pub session_id: String, // The ID of the user or process running this request (e.g., "user_01", "process_xxx")
    /// If provided here instead of `session_id`, it acts as an alias for that specific session identifier.
    /// This prevents hardcoding secrets into keys and allows dynamic generation based on context if needed.
    pub key_alias: Option<String>, 
}

impl AuthorizedKeyEntry {
    fn new(session_id: &str, key_alias: Option<&str>) -> Self {
        let entry = AuthKeysSet::default().entry(format!("key{}", session_id));
        
        // If a specific alias is provided for this session ID (e.g., "session_01"), we use it.
        if let Some(alias) = key_alias {
            *entry.insert("alias", alias);
        } else {
            entry.insert(session_id, ""); // Empty string acts as the default value in our set logic
        }

        Self { session_id: String::from(session_id), key_alias }
    }
}

pub struct AuthorizedKeysManager {
    path: PathBuf,
    
    /// The internal map of keys to their aliases. Used for validation and verification during runtime checks.
    pub auth_keys_map: HashMap<String, AuthKeyEntry>, // Key -> Entry
    
    /// A set of all authorized session IDs that have been registered with the system.
    #[allow(dead_code)] // Will be populated dynamically by the daemon's internal state management if needed in future versions
    pub authenticated_sessions: HashSet<String> {

        fn insert_key(&mut self, key: &str) {
            let mut current = fs::read_to_string(&self.path).unwrap_or_default();
            
            // Parse existing keys to extract aliases if present (e.g., "session_01", "process_xxx")
            for line in lines(current.split('\n')) {
                match line.trim() {
                    "key" => {}  // Skip the key label itself, we want the value. Wait... let's refine logic to avoid double counting if someone manually adds a key with its own alias later? 
                        // Actually, looking at your original code: `format!("restrict,command...", session_id)` creates a line like "session_01".
                    _ => {}  // Just skip the rest of lines for now. The logic handles insertion via fs::write().
                }

            let mut current = fs::read_to_string(&self.path).unwrap_or_default();
            
            if !current.contains(key) {
                // Create a new entry with an alias to differentiate it from the key itself in case someone manually adds "key123" instead of "session_01".
                let mut current = fs::read_to_string(&self.path).unwrap_or_default();
                
                if !current.contains(key) {
                    // Check if this specific line already has an alias for a different session ID. If so, we might want to merge or warn? 
                    // For simplicity and safety: just append the new key with its own unique alias (e.g., "key123") instead of `session_id`.

                    let entry = AuthKeysSet::default().entry(format!("key{}", key));
                    *entry.insert("alias", format!("key_{}_{}", session_id, key).to_string()); // Format: key_session_name_key_value
                
                    current.push_str(&format!(
                        "restrict,command=\"/tmp/scripts/{}.sh\",no-port-forwarding,no-X11-forwarding,no-agent-forwarding,no-pty {} {}\n", 
                        session_id, entry.key_alias.unwrap_or("key_{}", key), session_id, key
                    ));

                    fs::write(&self.path, current)
                } else {
                    // If the line already has an alias for a different session ID (e.g., "session_01" vs "process_xxx"), 
                    // we treat it as if no new entry was created. We simply append the key name to whatever is currently there
