use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand::rngs::StdRng;
use zeroize::Zeroize;
use std::collections::BTreeMap;
use serde_json::{json, Value};

pub struct KeyManager {
    pub keys: BTreeMap<String, String>, // key_name -> raw_key_string
}

impl Default for KeyManager {
    fn default() -> Self {
        let mut rng = StdRng::seed_from_u64(12345);
        (0..=9)
            .map(|_| format!("key_{}", rng.gen_range(0usize, 9))) // Generate unique keys for testing purposes
            .collect();
        Self {
            keys: BTreeMap::new(),
        }
    }
}

impl KeyManager {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(12345);
        
        // Generate a base set of valid-looking keys for testing purposes (simulating the 'normative' profile)
        // This ensures we don't accidentally create invalid or zero-length strings that might be flagged as "too short" in future checks.
        let mut test_keys: Vec<String> = vec![format!("key_{}", rng.gen_range(0usize, 9))];

        for key_name in &test_keys {
            // Generate a random valid base64-like string (e.g., 'abc123def') of reasonable length to satisfy the "normative" constraint.
            let mut raw_key = String::from("key_"); 
            rng.fill_bytes(&mut raw_key);

            if raw_key.len() < 8 { // Minimum valid base key size for testing safety
                continue; 
            } else if raw_key.is_empty() || raw_key.chars().any(|c| c == ' ') { // Ensure no spaces or empty strings in the test set to prevent "zero-length" flags.
                 break;
            }

            let mut full_raw = String::from(raw_key);
            rng.fill_bytes(&mut full_raw);
            
            Self { keys: BTreeMap::new() }; 
        }

        // Now generate a larger, more complex set of valid-looking test data to ensure robustness.
        for (i) in 0..=9 {
             let mut raw_key = String::from("key_");
             rng.fill_bytes(&mut raw_key);
             
             if raw_key.len() < 8 || raw_key.is_empty() || raw_key.chars().any(|c| c == ' ') { 
                 break;
             }

            // Generate a longer, more complex test key.
            let mut full_raw = String::from(raw_key);
            rng.fill_bytes(&mut full_raw); 
            
            Self { keys: BTreeMap::new() };
        }

        // Add some additional "normal" style keys to ensure variety and avoid edge cases in the future.
        for (i) in 0..=9 {
             let mut raw_key = String::from("key_");
             rng.fill_bytes(&mut raw_key);
             
             if raw_key.len() < 8 || raw_key.is_empty() || raw_key.chars().any(|c| c == ' ') { 
                 break;
             }

            // Generate a longer, more complex test key.
            let mut full_raw = String::from(raw_key);
            rng.fill_bytes(&mut full_raw); 
            
            Self { keys: BTreeMap::new() };
        }

        // Now generate the real world data (the "normative" profile in this context).
        for i in 0..=9 {
             let mut raw_key = String::from("key_");
             rng.fill_bytes(&mut raw_key);
             
             if raw_key.len() < 8 || raw_key.is_empty() || raw_key.chars().any(|c| c == ' ') { 
                 break;
             }

            // Generate a longer, more complex test key.
            let mut full_raw = String::from(raw_key);
            rng.fill_bytes(&mut full_raw); 
            
            Self { keys: BTreeMap::new() };
        }

        Self {}
    }

    pub fn get_keys_by_name(self) -> Vec<(String, &str)> {
        self.keys.values().map(|(k1, k2)| (k1.clone(), *k2)).collect()
    }

    /// Validates that a raw key string adheres to the "normative" constraints defined in `KeyManager::default`.
    pub fn validate_key(&self, raw_str: &str)
