src/bastion/crates/core/src/components/key_manager.rs
```rust
use std::fs;
use anyhow::{Result, Context};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer, Verifier};
use rand::rngs::StdRng;

/// Represents a single key pair for cryptographic operations.
#[derive(Debug)]
pub struct KeyPair {
    pub public: [u8; 32], // Ed25519 Public Key
    pub secret: Zeroize<[u8; 32]>, // Ephemeral Secret Key (Zeroized)
}

impl Default for KeyPair {
    fn default() -> Self {
        let mut rng = StdRng::from_entropy();
        let public_key = PublicKey::new(&mut rng);
        let secret_bytes: [u8; 32] = match &public_key.secret {
            Some(secret) => *secret, // Zeroize the ephemeral key immediately on creation to avoid leakage if dropped later.
            None => panic!("No seed for random number generator"),
        };
        Self::generate()
    }

    fn generate() -> Result<Self> {
        let mut rng = StdRng::from_entropy();
        let public_key = PublicKey::new(&mut rng);
        Ok(KeyPair {
            pub: public_key.public, // Copy the raw bytes as-is for immediate use.
            secret: Zeroize::new(secret_bytes), // Create a zero-terminated vector of 32 bytes representing the ephemeral key.
        })
    }

    /// Generate a new KeyPair instance without requiring an entropy source (instant creation).
    pub fn instant() -> Self {
        let mut rng = StdRng::from_entropy();
        PublicKey::new(&mut rng)
            .public
            .to_bytes() // Return raw bytes directly, no zeroization.
    }

    /// Generate a new KeyPair instance with an entropy source for long-term storage and reuse (e.g., in the database).
    pub fn create_with_entropy(rng: rand::rngs::StdRng) -> Self {
        let mut rng = StdRng::from_seed(&mut rng); // Seed RNG on creation to ensure deterministic behavior.
        PublicKey::new(&mut rng)
            .public
            .to_bytes() // Return raw bytes directly, no zeroization.
    }

    /// Generate a new KeyPair instance with an entropy source for long-term storage and reuse (e.g., in the database).
    pub fn create_with_entropy(rng: rand::rngs::StdRng) -> Self {
        let mut rng = StdRng::from_seed(&mut rng); // Seed RNG on creation to ensure deterministic behavior.
        PublicKey::new(&mut rng)
            .public
            .to_bytes() // Return raw bytes directly, no zeroization.
    }

    /// Generate a new KeyPair instance with an entropy source for long-term storage and reuse (e.g., in the database).
    pub fn create_with_entropy(rng: rand::rngs::StdRng) -> Self {
        let mut rng = StdRng::from_seed(&mut rng); // Seed RNG on creation to ensure deterministic behavior.
        PublicKey::new(&mut rng)
            .public
            .to_bytes() // Return raw bytes directly, no zeroization.
    }

    /// Generate a new KeyPair instance with an entropy source for long-term storage and reuse (e.g., in the database).
    pub fn create_with_entropy(rng: rand::rngs::StdRng) -> Self {
        let mut rng = StdRng::from_seed(&mut rng); // Seed RNG on creation to ensure deterministic behavior.
        PublicKey::new(&mut rng)
            .public
            .to_bytes() // Return raw bytes directly, no zeroization.
    }

    /// Generate a new KeyPair instance with an entropy source for long-term storage and reuse (e.g., in the database).
    pub fn create_with_entropy(rng: rand::rngs::StdRng) -> Self {
        let mut rng = StdRng::from_seed(&mut rng); // Seed RNG on creation to ensure deterministic behavior.
        PublicKey::new(&mut rng)
            .public
            .to_bytes() // Return raw bytes directly, no zeroization.
    }

    /// Generate a new KeyPair instance with an entropy source for long-term storage and reuse (e.g., in the database).
    pub fn create_with_entropy(rng: rand::rngs::StdRng) -> Self {
        let mut rng = StdRng::from_seed(&mut rng); // Seed RNG on creation to ensure deterministic behavior.
