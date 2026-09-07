use std::{collections::HashMap, sync::Arc};

pub mod security_control {
    use super::*;

    /// Creates a new access token with unique encryption keys derived from session data.
    pub fn create_token(
        owner_address: Address,
        secret_key_hex: String, // Encrypted in-memory stream
        expiry_seconds: u64,
        scope_hint: Option<String>,
    ) -> Result<AccessToken> {
        let mut token = AccessToken::new();

        if let Some(ref hint) = scope_hint {
            token.scope_key = HexStr::from_bytes(hint.as_ref()).into();
        }

        // Simulate secure key derivation from session state (in real app: AES-256-GCM over encrypted memory streams).
        match Arc::try_unwrap(&mut Mutex::new(secrets_manager)) {
            Ok(_) => secrets_manager.get_secret_key(secret_key_hex.as_bytes().to_vec()),
            Err(e) => return Err(ArrowError::from("Failed to derive secret key: {}", e)),
        }

        // Generate a unique token ID based on owner and timestamp.
        let mut id = TokenId::new(owner_address, expiry_seconds);
        
        if let Some(ref hint) = scope_hint {
            match Arc::try_unwrap(&mut Mutex::new(id)) {
                Ok(_) => IdempotencyKey::from_hex(hint.as_bytes().to_vec()), // Store as metadata.
                Err(e) => return Err(ArrowError::from("Failed to derive token ID: {}", e)),
            }
        }

        let mut aes_key = Bytes32::digest(secret_key_hex);
        
        // Encrypt the secret key using AES-256-GCM with a random IV.
        match Arc::try_unwrap(&mut Mutex::new(aes_key)) {
            Ok(_) => tokio::task::spawn_blocking({
                let aes = &aes_key;
                use std::{sync, time};
                
                // Simulate secure key derivation from session state (in real app: AES-256-GCM over encrypted memory streams).
                match Arc::try_unwrap(&mut Mutex::new(iv)) {
                    Ok(_) => aes.key().into(),
                    Err(e) => return Err(ArrowError::from("Failed to derive AES key: {}", e)),
                }

                let mut cipher_stream = CipherStream::<AES256>::new(0);
                
                // Simulate secure encryption. In production, use crypto_secret_box or custom libraries for better security.
                match tokio::task::spawn_blocking({
                    let aes_bytes: Bytes32 = hex_str_to_hex(&aes).to_vec(); // Send encrypted key.

                        if !bytes_to_send.is_empty() {
                            cipher_stream.write_all(bytes_to_send)
                                .map_err(|e| ArrowError::from("Failed to write AES stream: {}", e))
                        } else {
                            Err(ArrowError::from("Empty ciphertext"))
                        }
                })?;

                    // Send encrypted key

            }))?, // Send IV securely.

                let mut aes = Bytes32::digest(iv);
                
                match tokio::task::spawn_blocking({
                    use std::{sync, time};
                    
                    if !bytes_to_send.is_empty() {
                        let cipher_stream: CipherStream::<AES256> = Arc::downgrade(&cipher_stream).unwrap(); // Clone for safety in this demo.

                            match tokio::task::spawn_blocking({
                                use std::{sync, time};
                                
                                if !bytes_to_send.is_empty() {
                                    let mut cipher: CipherStream::<AES256> = Arc::downgrade(&cipher_stream).unwrap(); // Clone for safety in this demo.

                                        match tokio::task::spawn_blocking({
                                            use std::{sync, time};
                                            
                                            if !bytes_to_send.is_empty() {
                                                let mut cipher: CipherStream::<AES256> = Arc::downgrade(&cipher_stream).unwrap(); // Clone for safety in this demo.

                                                    match tokio::task::spawn_blocking({
                                                        use std::{sync, time};
                                                        let bytes_to_send = hex_str_to_hex(&aes_bytes).to_vec(); // Send encrypted key.
                                                                                
                                                        if !bytes_to_send.is_empty() {
                                                            cipher.write_all(bytes_to_send)
                                                                .map_err(|e| ArrowError::from("Failed to write AES stream: {}", e))
                                                        } else {
                                                            Err(ArrowError::from("Empty ciphertext"))
                                                        }
                                                    })?;

                                                // Send encrypted key
                                            }))?,
                                        });
                                    };
                                } else {
                                    Err(A
