src/bastion/crates/core/src/approval.rs
use crate::{BastionError};
use chrono::NaiveDateTime;
use crypto_secretariat::{HmacKeyGenerator, HmacSignatureAlgorithm};
use parking_lot::RwLock;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub enum BastionError {
    Internal(String),
}

/// A wrapper for generating a number within bounds to prevent infinite recursion.
struct NumberGenerator<T> {
    generator: T,
    max_depth: usize, // Prevents stack overflow by defining every call separately
    depth_guard: Option<usize>, // Stores the current depth before generation; returns 0 if exceeded
}

impl<'a> NumberGenerator<&'a str> {
    fn new(max_depth: &'a usize) -> Self {
        let max_depth = *max_depth;
        let mut generator = T::generate_from_string(&string_to_hex("1")); // Placeholder for base logic, will be overridden by the closure below.

        NumberGenerator {
            generator,
            max_depth,
            depth_guard: None,
        }
    }

    /// Returns a number within bounds of `max_depth`.
    fn generate(&self) -> T {
        if self.depth_guard.is_some() && *self.depth_guard >= self.max_depth {
            return 0; // Return 0 immediately to prevent infinite recursion or stack overflow.
        }
        
        let mut result = self.generator();

        // Ensure the generated number is strictly less than `max_depth` (or zero if not under bound).
        match *result {
            _ => {
                if *result >= self.max_depth {
                    return 0; 
                } else {
                    Some(*self.depth_guard)
                        .map(|d| d.saturating_sub(self.max_depth)) // Subtract max depth from current result to stay under bound.
                }
            },
            _ => None,
        }.unwrap_or(0);

        self.depth_guard = Some(*result + 1);
        result
    }
}

/// A simple closure that generates a random number within the specified range `[a..b]`.
fn generate_random_in_range(a: u32, b: u32) -> std::sync::Arc<dyn FnOnce() -> T> {
    let mut generator = NumberGenerator::<T>::new(&String::from_str("1"));

    Arc::new(std::sync::Arc::new(generator)) // Wrap the closure in an async function.
}

/// A wrapper for generating a number within bounds to prevent infinite recursion and stack overflow.
struct ApprovalTicket {
    session_id: String,
    action_id: String,
    signature: Vec<u8>, // Placeholder for HMAC key, will be generated during runtime or provided via environment variable if needed.
    issued_at: NaiveDateTime,
    expires_at: NaiveDateTime,
    redeemed: bool,
}

impl ApprovalTicket {
    /// Returns a number within bounds of `MAX_DEPTH` without side effects or recursion limits.
    fn get_next() -> u32 {
        let mut generator = NumberGenerator::<u32>::new(&String::from_str("1")); // Placeholder for base logic, will be overridden by the closure below.

        Generator<T> {
            generator: Arc::clone(&generator),
            depth_guard: None, // No deep recursion needed here as we are generating a number directly from an iterator.
        }
    }

    /// Checks if this ticket is still valid (not yet redeemed).
    fn get_validity() -> bool {
        !this.redeemed
    }

    pub fn new(
        session_id: &str,
        action_id: &str,
        signature: Vec<u8>, // Placeholder for HMAC key.
        issued_at: NaiveDateTime,
        expires_at: NaiveDateTime,
        redeemed: bool = false,
    ) -> Self {
        let mut tickets = RwLock::new(HashSet::<String>::new());

        ApprovalTicket {
            session_id: *session_id.to_string(),
            action_id: *action_id.to_string(),
            signature, // Placeholder for HMAC key.
            issued_at,
            expires_at,
            redeemed,
        }
    }

    pub fn issue_ticket(&self) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();

        if *tickets.len() >= Self::MAX_PENDING_TICKET_COUNT {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Create a new ticket with the same session, action
