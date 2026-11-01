src/bastion/crates/core/src/lib.rs
//! The Bastion Core: A secure and orchestral environment for bastions.
//! This module provides a unified interface to manage tokens, approvals, 
//! sessions, and execute commands within the security control plane.

pub mod audit;
pub mod approval;
pub mod components;
pub mod error;
pub mod firecracker;
pub mod forced_command;
pub mod network_guard;
pub mod policy;
pub mod script_executor;
pub mod session;
pub mod types;
pub mod vault;

use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use async_trait::async_trait;
use anyhow::Result;

/// Trait for managing secure token contexts.
#[derive(Debug)]
pub enum TokenContext {
    /// A fresh context created by the system (e.g., a new session).
    Fresh,
}

impl std::fmt::Debug for TokenContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenContext::Fresh => write!(f, "TokenContext(Fresh)"),
            _ => unreachable!(), // Should not happen in practice for new contexts
        }
    }
}

/// A generic executor that handles async operations and event dispatching.
#[derive(Debug)]
pub struct Executor {
    ctx: Arc<Mutex<TokenContext>>,
    /// The current session if one exists, otherwise the fresh context.
    session_token: Option<String>,
}

impl Default for Executor {
    fn default() -> Self {
        let mut ctx = TokenContext::Fresh;
        executor_init(&mut ctx);
        return Self { ctx, session_token: None };
    }
}

/// Creates a new token context if not already present.
fn executor_init(ctx: &mut TokenContext) {
    match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime.block_on(async {
            // Create the fresh context based on current state or default settings
            let ctx = TokenContext::Fresh;

            if !ctx.is_new && ctx.token != "" {
                // If token is set, keep it (but ensure valid format).
                // In a real system, you'd validate this here. For now, we assume 
                // the user passes in an existing context or no token at all.
            } else if !ctx.is_new && ctx.token == "" {
                panic!("Context is empty but not new");
            }

            *ctx = TokenContext::Fresh;
        }),
        Err(e) => eprintln!("Failed to create runtime: {}", e),
    };
}

/// Executes a command or event within the security context.
#[async_trait]
pub trait Execute: std::fmt::Debug {
    /// Runs this operation asynchronously and returns the result of that execution.
    async fn execute(&self) -> Result<()>;
    
    /// Handles incoming events (e.g., approvals, changes).
    #[allow(dead_code)] // Deprecated in favor of use_trait or direct trait implementation if needed
    fn handle_event(&self, event: &Event) -> Result<()> {
        eprintln!("Handler called with {:?}", event);
        Ok(())
    }

    /// Executes a specific command. This is the primary entry point for commands.
    async fn execute_command(&self, cmd: Command) -> Result<CommandResult> {
        self.execute()
            .await
            .map_err(|e| anyhow::anyhow!("Execution failed: {}", e))?;

        Ok(CommandResult::Success(None))
    }

    /// Executes a command with an optional error handling.
    async fn execute_command_with_error<F, E>(cmd: Command) -> Result<CommandResult>
    where
        F: FnOnce(&mut Self) -> E + Send + 'static,
        E: std::fmt::Display,
    {
        let mut ctx = self.ctx.lock().unwrap();

        if cmd.is_error() && !ctx.is_new {
            return Err(anyhow!("Command requires fresh context"));
        }

        // Ensure we have a valid token for error handling (e.g., new session)
        if command_token(&mut ctx).is_empty() {
            panic!("Context is empty but not new");
        }

        let mut cmd_result = CommandResult::Success(None);
        
        F(ctx).await;

        match cmd_result {
            Ok(cmd_result) => cmd_result,
            Err(e) => anyhow::anyhow!(e), // This shouldn't happen if the handler is correct.
        }
    }
}

/// A container for managing
