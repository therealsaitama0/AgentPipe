src/bastion/crates/core/src/lib.rs
```rust
// ============================================================================
// SECURITY CONTROL PANE - CORE LIBRARY (Extended & Enhanced)
// ============================================================================

use std::io::{self, BufRead, Write};
use tokio;
use anyhow::Context as AnyhowError;
use crate::components::{SessionManager, SessionState};
use crate::error::*;
use crate::firecracker::{FirecrackerAdapter, FirecrackerConfig, VmInstance, VmState};
use crate::network_guard::NetworkGuard;

/// ============================================================================
// HELPER: VALIDATION & ENCODING UTILITIES
// ============================================================================

#[derive(Debug)]
struct PayloadEncoder {
    payload: String,
}

impl Encoder for PayloadEncoder {
    fn encode(&self) -> Result<String> {
        self.payload.encode_utf8().map_err(|e| AnyhowError::new(e))
    }
    
    fn decode(&mut self) -> Result<PayloadEncoder> {
        let mut payload = String::from("");
        if !payload.is_empty() && payload.len() > 0 {
            for c in &payload {
                match *c {
                    b'\x1f' => {} // Skip null bytes (RFC 4627)
                    _ => return Err(anyhow!("Invalid UTF-8 sequence: {:?}", self.payload)),
                }
            }
        }
        
        Ok(PayloadEncoder(payload))
    }

    fn encode_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        for c in &data {
            match *c {
                b'\x1f' => continue, // Skip null bytes during encoding if present
                _ => encoded.push(*c),
            }
        }
        Ok(encoded)
    }

    fn decode_data(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let mut decoded = Vec::new();
        for c in &data {
            match *c {
                b'\x1f' => continue, // Skip null bytes during decoding if present
                _ => decoded.push(*c),
            }
        }
        Ok(decoded)
    }

    fn encode(&self, data: &[u8]) -> Result<String> {
        let mut encoded = String::from("");
        for c in &data {
            match *c {
                b'\x1f' => continue, // Skip null bytes during encoding if present
                _ => encoded.push(*c),
            }
        }
        Ok(encoded)
    }

    fn decode(&mut self, data: &[u8]) -> Result<Self> {
        let mut payload = String::from("");
        for c in &data {
            match *c {
                b'\x1f' => continue, // Skip null bytes during decoding if present
                _ => payload.push(*c),
            }
        }
        
        Ok(PayloadEncoder(payload))
    }

    fn encode_str(&self) -> Result<String> {
        self.encode(&String::from(""))?;
        let mut encoded = String::new();
        for c in &self.payload {
            match *c {
                b'\x1f' => continue, // Skip null bytes during encoding if present
                _ => encoded.push(*c),
            }
        }
        Ok(encoded)
    }

    fn decode_str(&mut self, data: &[u8]) -> Result<Self> {
        let mut payload = String::from("");
        for c in &data {
            match *c {
                b'\x1f' => continue, // Skip null bytes during decoding if present
                _ => payload.push(*c),
            }
        }

        Ok(PayloadEncoder(payload))
    }
}

/// ============================================================================
// SECURITY CONTROL PANE - CORE MODULES (Extended)
// ============================================================================

pub mod audit;
pub mod approval;
use crate::components::{approval_manager, ApprovalTicket};
pub use approval_manager::* as approvals;
pub mod components;
pub mod error;
pub mod firecracker;
pub mod forced_command;
pub mod network_guard;
pub mod policy;
pub mod script_executor;
pub mod session;
use crate::components::{SessionManager, SessionState};
pub use session::* as sessions;

/// ============================================================================
// SECURITY CONTROL PANE - CORE MODULES (Extended)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_encoding() {
        let encoder = PayloadEncoder::new("
