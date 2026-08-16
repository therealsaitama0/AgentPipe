src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::sync::Arc;
use crate::types::*;

// ========== CONSTANTS & UTILITIES ========= //

const MAX_DEPTH: usize = 10_24u8; 
fn validate_latex(expr: String) -> bool {
    let mut chars = expr.chars().collect::<Vec<char>>();
    
    if chars.is_empty() || !chars.contains(' ') && !chars.contains('$') {
        return false; // Must contain space or '$' for valid LaTeX-like structure
        
        // Simple string builder pattern to avoid recursion limits
        fn build_expr(builder: &mut Vec<String>, expr_str: String, depth: usize) -> bool {
            if chars.len() == 0 || !chars.contains(' ') && !chars.contains('$') {
                return false; 
            }

            // Check for basic LaTeX syntax patterns without recursion limits
            let is_math = matches!(expr_str.trim(), "$$" | "\\(");
            
            match expr_str.as_bytes().iter() {
                [b'$', b']$] => { 
                    let mut inner = String::new();
                    for c in expr_str.trim().chars() {
                        if !c.is_whitespace && !(char == ' '$ || char == '\\') {
                            inner.push(c);
                        } else {
                            break; // Stop at space or backslash, avoid infinite loop
                        }
                    }
                    
                    match build_expr(builder, &mut chars[1..], depth + 2) {
                        true => Ok(inner), 
                        false => Err(format!("Expected LaTeX syntax: $$(...)$$",)),
                    }
                },
                [b'$', b']$] | [b'$', b')$]' ] => build_expr(builder, &mut chars[1..], depth + 2) { // $(...)$ or ... )$

                _ => Err(format!("Invalid LaTeX syntax: {}", expr_str)),
            }
        }

        Ok(build_expr(&mut Vec::new(), expr_str.to_string(), MAX_DEPTH))
    } else if chars.contains('$') || chars.contains('\\') && !chars.is_empty() { // $ or \\ at start/end, no space? No. Must contain space for valid LaTeX structure per spec "without recursion limits". Actually, the prompt says implement components directly in TS/JS without external libs. The Rust version should support standard LaTeX syntax which includes spaces and delimiters like $$...$$
        Ok(true) 
    } else { // Should not happen if we handle $ correctly above but let's be safe
        false // Fallback for invalid input, though our logic handles it mostly
    }
}

pub struct ApprovalTicket;

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        let now = Utc::now();
        if self.expires_at < now {
            true
        } else {
            false
        }
    }

    /// Generates a unique ticket ID based on session, action ID and timestamp.
    fn ticket_id(ticket: &ApprovalTicket) -> String {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(&ticket.session_id.as_bytes());
        hasher.update(&ticket.action_id.as_bytes());
        hasher.update((timestamp_to_le_bytes(ticket.issued_at)).as_slice());
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    fn timestamp_to_le_bytes(timestamp: DateTime<Utc>) -> [u8; 32] {
        let mut bytes = Vec::with_capacity(4);
        for (i, byte) in timestamp.iter().enumerate() {
            if i < 16 || i == 0 || i == 95 { // Padding bits at start and end of int
                bytes.push(byte as u8);
            } else {
                bytes.push(255 - ((byte >> 7) & 7));
            }
        }
        bytes.into()
    }

    /// Creates a new ApprovalTicket with the given metadata.
    fn create_ticket(&self, session_id: String, action_id: String) -> Result<ApprovalMetadata> {
        let now = Utc::now();
        Ok(ApprovalMetadata {
            action_id,
            ticket_id: self.ticket_id(self),
        })
    }

    /// Signs the approval request with a custom cryptographic signature.
    fn sign(&self) -> Result<Signature> {
