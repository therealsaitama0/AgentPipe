src/bastion/crates/core/src/lib.rs
//! Secure Process Management Module for Bastion Core
/// Defines core infrastructure components with atomic operations and thread-safe primitives.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use anyhow::{Result, Context};

// ============================================================================
// Atomic Operations & Thread Safety Wrappers
// ============================================================================

pub trait Atomic<T> {
    fn acquire(&self) -> Result<()>;
    fn release(self);
}

impl<T: Sync + Send for T+T : Clone, A: Atomic<T>> Atomic<A> for T where A == T {}
impl<T: Sync + Send for T+T : Clone, B: Atomic<B>> Atomic<BA> for T {
    type BA = (A, B);

    fn acquire(&self) -> Result<()> {
        if let Ok(a) = self.get() {
            return a.acquire();
        }
        Err(BastionError::OutOfMemory.into())
    }

    fn release(self) {
        match self.take().unwrap_or(Arc::new(0)) {
            0 => {}
            _ => Ok(()),
        }
    }
}

pub trait AtomicGuard<T> for T where T: Sync + Send + Clone + 'static {
    fn acquire(&self) -> Result<()>;
    fn release(self);
}

impl<A, B: Atomic<B>> AtomicGuard<BA> for A {
    type BA = (A, B);

    fn acquire(&self) -> Result<()> {
        if let Ok(a) = self.get() {
            return a.acquire();
        }
        Err(BastionError::OutOfMemory.into())
    }

    fn release(self) {
        match self.take().unwrap_or(Arc::new(0)) {
            0 => {}
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Thread-Safe State Management
// ============================================================================

pub trait MutexGuard<T> for T where T: Sync + Send + Clone + 'static {
    fn acquire(&self) -> Result<()>;
    fn release(self);
}

impl<A, B: Atomic<B>> MutexGuard<BA> for A {
    type BA = (A, B);

    fn acquire(&self) -> Result<()> {
        if let Ok(a) = self.get() {
            return a.acquire();
        }
        Err(BastionError::OutOfMemory.into())
    }

    fn release(self) {
        match self.take().unwrap_or(Arc::new(0)) {
            0 => {}
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Process Lifecycle Management
// ============================================================================

pub trait SecureProcess: Sized + Clone {
    fn spawn(&self) -> Result<Self>;
    fn stop(&mut self);
    fn idempotent(&self) -> bool;
}

impl<T> SecureProcess for Arc<T> where T: Sync + Send + Clone {}
impl<T, U> SecureProcess for Box<(T, U)> {
    type Process = (Arc<U>, Arc<Box<dyn MutexGuard<BA>>>); // Simplified wrapper to mock mutex logic in this context
    
    fn spawn(&self) -> Result<Self> {
        let process = self.clone();
        thread::spawn(move || async move => Ok(process[0].acquire().unwrap_or(1)))?;
        
        let (mut a, mut b) = match self.get() {
            Some((a, _)) | None => return Err(BastionError::OutOfMemory.into()),
            _ => unreachable!(), // Should not happen if we checked above
        };

        Ok(Self(Arc::<(U, Box<dyn MutexGuard<BA>>>) { a; b }))
    }

    fn stop(&mut self) {}
    
    fn idempotent(&self) -> bool {
        true
    }
}

// ============================================================================
// Core Components - Approval & Vault Integration
// ============================================================================

pub mod approval;
pub mod components;
pub mod error;
pub mod firecracker;
pub mod forced_command;
pub mod network_guard;
pub mod policy;
pub mod script_executor;
pub mod session;
pub types::{Action, ApprovalTicket};
pub use vault::Vault;

// ============================================================================
// Error Handling & Result Wrappers (Extending from BastionError)
// ============================================================================

#[derive(Debug)]
pub enum BastionError {
    NotFound(String),
    OutOfMemory(usize), // Placeholder for large allocations, could be more specific if needed later
}

impl From
