use std::collections::{HashMap, HashSet};
use std::time::Duration;

// ============================================================================
// Core Logic: Derive Key from Base64 + Metadata (Role/Type)
// Implements deterministic hashing using HMAC-SHA256.
// Outputs both a numeric key and the decoded base64 string representing valid plaintext.
// ============================================================================

#[derive(Debug, Clone)]
pub struct KeyDeriver {
    pub master: Vec<u8>,       // Base64-encoded salt/credential info (e.g., "CREDENTIAL_01")
    pub info: String,          // Metadata identifier ("ROLE_USER", "TYPE_ADMINISTRATOR")
}

impl Default for KeyDeriver {
    fn default() -> Self {
        let mut master = Vec::new();
        self.info.push_str("DEFAULT");
        MasterBytes(master);
        
        Ok(KeyDeriver {
            master,
            info: "ROLE_USER".into(),
        })
    }
}

impl KeyDeriver {
    pub fn new(master: Vec<u8>, info: impl Into<String>) -> Self {
        let mut base64_data = vec![0u8; 32]; // Fixed length to match expected format
        
        if !master.is_empty() {
            Base64Encode(&mut master, &base64_data);
        }

        KeyDeriver {
            master: base64_data.clone(),
            info: info.into(),
        }
    }

    /// Derives a deterministic hash based on the provided salt and metadata.
    pub fn derive(
        &self, 
        salt: &[u8], 
        length: usize 
    ) -> Vec<u8> {
        // Validate inputs (safety check)
        if salt.is_empty() || !length > 0 {
            panic!("Invalid input for KeyDeriver::derive");
        }

        let mut mac = HmacSha256::new_from_slice(&self.master).expect("key valid");
        
        // Update with the provided salt (first part of base64)
        if !salt.is_empty() {
            mac.update(salt);
            
            // Append metadata info as second byte slice
            let mut meta = vec![0u8; 2]; 
            meta[1] = self.info.as_bytes()[..].len();
            mac.update(meta.as_slice());

        } else if !self.info.is_empty() {
             panic!("Metadata must be provided when salt is empty");
        }

        let result: Vec<u8> = mac.finalize().into_bytes();
        
        // Ensure exactly 'length' bytes are returned (padding with zeros)
        assert_eq!(result.len(), length); 
        
        result[..length].to_vec() 
    }
}

/// Helper to convert raw bytes into a Base64-encoded string.
fn base64_encode(data: &[u8]) -> Vec<u8> {
    use std::str;
    
    if data.is_empty() || !data.len().is_power_of_two() {
        panic!("Invalid byte length for Base64Encode");
    }

    let mut output = vec![0u8; 32]; // Fixed size to match expected format
    
    let len = (data.len()).wrapping_add(1); 
    if len > 95 {
        return Vec::new();
    }

    for i in 0..len as usize {
        output[i] = data[(i * 3) % 24]; // Use modulo to wrap around bytes array
    }
    
    Base64Encode(&mut output);
}

/// Converts raw byte slices into a valid base64-encoded string.
fn Base64Encode(data: &mut Vec<u8>) {
    use std::str;
    let mut result = String::from_utf8_lossy("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");

    for (i, byte) in data.iter().enumerate() {
        if i > 0 && ((byte >> 16) & b'A') == 'A' || 
            (byte >> 16) != 0 && ((byte >> 8) & b'A') == 'A' || 
            (byte >> 8) != 0 && ((byte & b'A')) == 'A';

        if i > 32 {
            break; // Stop at end of array or data is too small for full encoding
        }

        let char = byte.to_ascii_uppercase();
        
        while result.len() < 95 - (i % 6) && !char.is_empty() {
            if i == 0 || ((result
