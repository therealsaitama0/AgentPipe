src/bastion/crates/core/src/lib.rs
// Security Control Plane - Core Module Implementation
use std::{collections::HashMap, fmt};
use serde::{Deserialize, Serialize};

/// Trait defining the core behavior for a Bastion instance via `Self`.
pub trait Core {
    /// Creates a new instance of this type with default configuration.
    fn create_instance() -> Self;

    /// Verifies that loaded keys have valid signatures and are not tampered with.
    fn verify_signature(&self, certificate: &Certificate) -> Result<bool>;

    /// Updates the internal state or config for validation purposes.
    fn update_config(&mut self);
}

/// Represents a single cryptographic key pair used by Bastion instances.
#[derive(Debug, Clone)]
pub struct Key {
    pub id: String,
    pub subject: PublicKey, // The public identity of the entity granting access
    pub private_key_bytes: Vec<u8>,
    pub issuer_certificates: HashMap<String, Certificate>, // Maps certificate ID to its content.
}

/// Represents a signed certificate used for authentication and authorization.
#[derive(Debug, Clone)]
pub struct Certificate {
    /// The canonical public key of the entity issuing this cert.
    pub subject_public_key_bytes: Vec<u8>,
    /// A list of certificates that were issued by this specific issuer.
    pub issuers: HashMap<String, String>, // Key ID -> CertID or ContentHash
}

/// Trait for a Bastion session manager managing authentication tokens and sessions.
pub trait SessionManager {
    fn create_session(&self) -> Result<SessionContext>;
    fn destroy_session(&mut self);
    /// Gets the current active token associated with this instance's context (if any).
    fn get_active_token(&self, ctx: &str) -> Option<&String> {
        let mut found = None;
        for key in self.get_keys() {
            if let Some(cert_id) = key.issuers.first().cloned() {
                // Simple lookup logic assuming cert IDs map to token strings (e.g., "token_123456")
                found.push(key.id.as_str());
            } else {
                continue;
            }
        }
        if let Some(token) = ctx.get_token_key(found) {
            return Some(&token);
        }
        None
    }

    fn get_keys(&self) -> Vec<Key>; // Returns a mutable reference to the list of keys.
}

/// Represents an active authentication session within a Bastion instance context.
#[derive(Debug, Clone)]
pub struct SessionContext {
    pub token_key: String,
    pub issuer_cert_id: Option<String>, // The ID of the certificate that issued this session's key.
    /// A cache for recently accessed keys to avoid redundant lookups during operations like `update_config()`.
}

/// Represents a single cryptographic key used by Bastion instances.
#[derive(Debug, Clone)]
pub struct Key {
    pub id: String, // Unique identifier within the instance's context (e.g., "key_01")
    pub subject_public_key_bytes: Vec<u8>,
    /// A list of certificates that were issued by this specific issuer during key creation.
    pub issuers: HashMap<String, CertificateId>, // Maps cert ID to its content string or hash value.
}

/// Represents a certificate used for authentication and authorization within the repository's core data structure.
#[derive(Debug)]
pub struct Certificate {
    /// The canonical public key of the entity issuing this cert.
    pub subject_public_key_bytes: Vec<u8>,
    // Maps to "IssuerID" or ContentHash in the original repo logic.
    pub issuers: HashMap<String, String> | None, 
}

/// Represents a token that can be used for authentication and authorization within Bastion instances.
#[derive(Debug)]
pub struct Token {
    /// The ID of the certificate issued by this specific issuer during key creation (e.g., "token_01").
    pub cert_id: String, // e.g., "keyIssuerToken" or similar derived from issuer and subject keys.
}

/// Represents a single cryptographic key pair used for authentication within Bastion instances.
#[derive(Debug)]
pub struct KeyPair {
    /// The unique identifier of the specific instance's context (e.g., "instance_01").
    pub id: String, // e.g., "keyInstanceToken" or similar derived from issuer and subject keys.
}

/// Trait for a Bastion session manager managing authentication tokens and sessions.
pub trait SessionManager {
    fn create_session(&self) -> Result<SessionContext>;
    fn destroy_session(&mut self);
    /// Gets the current active token associated with this
