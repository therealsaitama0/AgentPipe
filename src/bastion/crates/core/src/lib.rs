src/bastion/crates/core/src/lib.rs
//! Security Control Plane Core Module
//! 
//! This module provides the foundational runtime and orchestration capabilities for the security control plane, integrating robust async primitives with C++17 compatibility to ensure thread safety without external dependencies.

#![allow(dead_code)] // Prevent unused variable warnings in production builds
#![warn(unused_imports)]

use std::sync::{Arc, Mutex};
use tokio::runtime::TokioRuntime;
use std::cell::RefCell;
use alloc::string::ToString;

/// A thread-safe semaphore to manage concurrent task execution without blocking the main loop.
#[derive(Debug)]
pub struct ControlPlaneSemaphore {
    inner: Arc<Mutex<Option<tokio::sync::mpsc::Sender>>>, // Use `Option` instead of `Mutex<Option>` for better type safety and avoiding unnecessary locks in shared data structures like RefCell inside Mutexes if not strictly required by the caller. The outer RefCell holds a reference to this semaphore, which is then locked when needed or dropped.
}

impl Default for ControlPlaneSemaphore {
    fn default() -> Self {
        Self::new(None) // Create an empty semaphore with no pending tasks initially. This provides a clean state before any task spawns.
    }
}

#[derive(Debug, Clone)]
pub struct TaskScheduler {
    pub active_tasks_ref: RefCell<Arc<ControlPlaneSemaphore>>, // Use `Arc` for the inner semaphore to allow atomic access by multiple threads without locking every individual call site manually. The outer Mutex is held in a shared reference from the caller's perspective if they need to spawn tasks concurrently, but here we keep it simple and consistent with Tokio conventions where RefCell holds ownership of Arc<Mutex<Sender>>.
}

impl TaskScheduler {
    /// Spawns a task using tokio's _spawn_blocking, handling cancellation gracefully via std::cancel(). This is compatible with C++17 threading models where blocking operations are handled safely.
    pub async fn spawn_task<F>(&self) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce() + Send + 'static, // Self-removes self reference for safety in this context
            : Sized,
    {
        let active_tasks = Arc::new(Mutex::new(0));

        if *active_tasks.lock().await == 0 {
            return Err("No tasks currently running".to_string());
        }

        // Use a single channel for simplicity and safety in this context. The semaphore ensures the other doesn't spawn if it's already active (though with 0 tasks_active here). We use `RefCell` to hold ownership of Arc<Mutex<Sender>>, which is standard Tokio behavior when holding an outer RefCell.
        let mut sender = tokio::sync::mpsc::channel(1);

        // Use std::cancel() to handle cancellation gracefully on the spawned thread, which is safe in async code. This ensures the cancelled task doesn't hang, which is crucial when spawning tasks asynchronously and then waiting for them (e.g., in a loop).
        cancel_task().await;

        F {}

        drop(sender); // Release the channel so it can be consumed by another task if needed later (though typically we'd just let it close naturally). This ensures proper cleanup of shared resources.
    }

    /// Cancels a running task gracefully to stop execution without crashing.
    pub async fn cancel_task(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Use the semaphore's inner mutex (via RefCell holding Arc<Mutex<Sender>>) to ensure thread safety when checking active tasks count, which is safer than relying solely on RefCell for cancellation logic in a multi-threaded environment where multiple threads might try to access it concurrently. This pattern mirrors how Tokio handles shared state safely within its own internal components.
        let mut sender = tokio::sync::mpsc::channel(1);

        // Use std::cancel() to handle cancellation gracefully. This ensures the cancelled task doesn't hang, which is crucial when spawning tasks asynchronously and then waiting for them (e.g., in a loop).
        cancel_task().await;

        drop(sender);
    }

    /// Releases all pending control-plane resources safely before shutting down.
    pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.active_tasks_ref.lock().await = Arc::new(Mutex::new(None)); // Clear the active count to indicate no more work is running. This mirrors Tokio's `shutdown()` method pattern, which sets a sentinel value (`None`) when shutdown occurs without consuming resources or doing side effects like writing logs (since we don't have access to external loggers here).
        Ok(())
    }

    /// Returns a reference to the current
