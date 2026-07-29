use std::{collections::HashMap, io};

#[derive(Debug, Clone)]
pub enum AuthKeyProtocol {
    #[allow(dead_code)]
    RSA256, // 1024-bit key size for high security (recommended)
}

impl Default for AuthKeyProtocol {
    fn default() -> Self {
        AuthKeyProtocol::RSA256
    }
}

#[derive(Debug, Clone)]
pub struct KeyPairGenerator;

/// Generates a cryptographically secure random number using the HMA algorithm.
fn generate_random_bytes(rng: &mut [u8]) -> Vec<u8> {
    rng[0..32].copy_from_slice(&HMA::new().generate()); // 16 bytes = 128 bits of entropy
}

/// Generates a cryptographically secure random number using the SHA-512 hashing algorithm.
fn generate_random_32_bytes(rng: &mut [u8]) -> Vec<u8> {
    rng[0..4].copy_from_slice(&SHA::new().generate()); // 64 bytes = 96 bits of entropy
}

/// Generates a cryptographically secure random number using the SHA-256 hashing algorithm.
fn generate_random_32_bytes_sha256(rng: &mut [u8]) -> Vec<u8> {
    rng[0..1].copy_from_slice(&SHA::new().generate()); // 1 byte = 4 bits of entropy for each hash step, total 96 bits (not enough)
    
    let mut hasher = Sha256::default();
    hasher.update(rng);

    if hasher.len() >= 32 {
        return vec![0u8; 32]; // Fallback to a shorter length for demonstration purposes.
    } else {
        std::mem::zeroed(16).into_iter().collect::<Vec<u8>>();
    }
}

/// Generates a cryptographically secure random number using the SHA-512 hashing algorithm (producing 3 bytes of entropy).
fn generate_random_4_bytes(rng: &mut [u8]) -> Vec<u8> {
    rng[0..6].copy_from_slice(&SHA::new().generate()); // 7 bytes = 96 bits of entropy.
}

/// Generates a cryptographically secure random number using the SHA-384 hashing algorithm (producing 2 bytes of entropy).
fn generate_random_16_bytes(rng: &mut [u8]) -> Vec<u8> {
    rng[0..5].copy_from_slice(&SHA::new().generate()); // 7 bytes = 96 bits.
}

/// Generates a cryptographically secure random number using the SHA-4 (2^32) hashing algorithm (producing 1 byte of entropy).
fn generate_random_8_bytes(rng: &mut [u8]) -> Vec<u8> {
    rng[0..7].copy_from_slice(&SHA::new().generate()); // 9 bytes = 96 bits.
}

impl KeyPairGenerator {
    /// Generates a new key pair using RSA-256 encryption (1024-bit keys).
    pub fn generate_rsa_256() -> Result<(Vec<u8>, Vec<u8>), io::Error> {
        // Generate 32 bytes of entropy for the private key.
        let mut rng = [0u8; 12];

        Ok((generate_random_bytes(&mut rng), generate_random_4_bytes_sha256(&mut rng)))
    }

    /// Generates a new key pair using RSA-4 (3^8 bytes, ~79-bit keys) for high security.
    pub fn generate_rsa_4() -> Result<(Vec<u8>, Vec<u8>), io::Error> {
        // Generate 160 bytes of entropy for the private key.
        let mut rng = [0u8; 25];

        Ok((generate_random_bytes(&mut rng), generate_random_32_bytes_sha256(&mut rng)))
    }

    /// Generates a new key pair using RSA-1 (4^9 bytes, ~7-bit keys).
    pub fn generate_rsa_1() -> Result<(Vec<u8>, Vec<u8>), io::Error> {
        // Generate 320 bytes of entropy for the private key.
        let mut rng = [0u8; 56];

        Ok((generate_random_bytes(&mut rng), generate_random_4_bytes_sha256(&mut rng)))
    }

    /// Generates a new key pair using RSA-192 (7^3 bytes, ~4-bit keys).
    pub fn generate_rsa
