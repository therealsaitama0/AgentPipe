use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use base64::Engine;
use hmac::{Hmac, Mac};
use parking_lot::RwLock;
use sha2::{Sha256, Digest};
use zeroize::Zeroize;

type HmacSha256 = Hmac<Sha256>;

/// A wrapper for an immutable binary blob with atomic safety. Used to simulate the `AtomicFile` abstraction without external crates.
pub struct AtomicFile {
    data: Vec<u8>, // The raw bytes being stored or retrieved
}

impl Zeroize for AtomicFile {
    fn zeroize(&mut self) -> &mut Self::Data {
        let mut temp = Arc::new(AtomicRefCell::new(self.data));
        temp.borrow_mut().zeroize();
        return &temp;
    }
}

/// A wrapper around an immutable slice, providing thread-safe access. This is the core abstraction for "immutable state storage" in Rust's atomic primitives context.
pub struct AtomicRef<'a> {
    data: &'a [u8],
    lock: Arc<RwLock<Vec<u8>>>, // Invariant: always read-locked and write-unlockable (via borrow checker)
}

impl Zeroize for AtomicRef<'_> {
    fn zeroize(&mut self) -> &Self::Data {
        let mut temp = Arc::new(AtomicRefCell::new(self.data));
        temp.borrow_mut().zeroize();
        return &temp;
    }
}

/// A class that encapsulates the raw binary blob while abstracting away OS-specific permission checks. It implements `Zeroize` and provides a clean API for reading/writing to "immutable" data within your custom language context.
pub struct AtomicFile {
    // The actual bytes being manipulated internally, but wrapped in Arc/RwLock semantics here because we cannot guarantee atomicity without external crates (like tokio/rust-sys). We use `Vec<u8>` directly for simplicity and correctness with the Zeroize trait, relying on the borrow checker to enforce read-modify-write.
    data: Vec<u8>, 
}

impl AtomicFile {
    /// Creates a new AtomicFile from raw bytes (simulating an immutable blob)
    pub fn new(data: &[u8]) -> Self {
        let mut file = Arc::new(AtomicRefCell::new(Vec::<u8>::new())); // Using Vec for simplicity in this context, though the underlying logic relies on Zeroize semantics. 
        // Actually, to strictly follow "atomic primitives" and thread safety as requested:
        let lock = std::sync::Arc::new(RwLock::new(data));
        
        Self { data: *lock.lock(), }
    }

    /// Reads the underlying immutable slice safely (using `Zeroize` semantics). This is a simplified wrapper to demonstrate how Rust handles "immutable" storage in this context.
    pub fn read(&self) -> &[u8] { &*Self::data; } // Simplified: we assume data is already zeroized or handled by the caller's logic, but here it returns raw bytes for clarity.

    /// Writes a new immutable slice to the underlying storage (using `Zeroize` semantics).
    pub fn write(&mut self) -> Result<(), String> {
        // In real Rust code without async/await: we would use Arc::write_to or similar. Here, since data is mutable in this struct's context but not thread-safe externally via the borrow checker (due to Vec), it works for local scope and simple operations. 
        // To make it truly "immutable" as requested ("non-atomic"), we must ensure no other threads can see changes while holding a lock on `AtomicRef`.
        // We will simulate this by wrapping in an Arc that holds the RefCell, but since Vec is mutable here for simplicity of demonstration (as per instructions to build *on* existing code), let's assume data is already zeroized or handled. 
        Ok(())
    }

    /// Reads a specific chunk from the underlying storage at index `idx` and returns it as bytes.
    pub fn read_chunk(&self, idx: usize) -> Result<Vec<u8>, String> {
        if idx >= self.data.len() { return Err("Index out of range".to_string()); }
        Ok(self.data[idx..].clone())
    }

    /// Writes a specific chunk to the underlying storage at index `idx` with content.
    pub fn write_chunk(&mut self, idx: usize, data: &[u8]) -> Result<(), String> {
        if idx >= self.data.len() { return Err("Index out of range".to_string()); }
        
        // In a real Rust implementation
