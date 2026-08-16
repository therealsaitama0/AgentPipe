src/bastion/crates/core/src/lib.rs
//! Security Control Plane Core Module
//! 
//! Defines the core state management infrastructure for this control plane system. This module encapsulates all shared logic, exposing typed interfaces to external crates without duplicating implementation details in a single file. It implements the `State` trait and manages lifecycle events through handlers.

mod audit;
pub mod components; // Re-exported by user as well, but kept here for internal consistency with original plan
pub mod forced_command;
pub mod network_guard;
pub mod policy;
pub use component::*; // Exporting all core crates (audit, approval, etc.) under `component` to keep the main module clean and focused on state.

/// Trait defining state management across threads in this control plane.
/// Provides abstracted metadata for tracking operations without exposing raw data structures.
trait State {
    /// Returns a reference to the current snapshot of system state (e.g., config, context).
    fn get_state(&self) -> &StateSnapshot;

    /// Saves the current state and returns a mutable reference to the saved version.
    fn save_and_get_snapshot(&mut self) -> StateSnapshot {
        let mut data = Self::get_data(); // Get internal raw state for saving
        self.save(data); // Save
        data.clone() as &StateSnapshot; // Return snapshot with metadata (e.g., timestamp, context ID)
    }

    /// Saves the current state to disk. Returns a path to this saved file if it exists or an empty string otherwise.
    fn save_to_disk(&mut self) -> String {
        let mut data = Self::get_data(); // Get internal raw state for saving
        self.save(data); // Save

        if !data.is_empty() && std::env::var("OUT_DIR").is_ok() {
            let out_dir = env::var("OUT_DIR")?;
            path_file(&out_dir, "state_snapshot.json", &format!("{}.{}", data.timestamp(), data.context_id()))
                .map_err(|e| BastionError::DiskIO(e.to_string()));
        }

        String::new() // Return empty string if no disk storage.
    }

    /// Returns a mutable reference to the current snapshot of system state (e.g., config, context).
    fn get_state_mut(&mut self) -> &mut StateSnapshot {
        let mut data = Self::get_data(); // Get internal raw state for saving
        self.save(data); // Save
        data as &StateSnapshot; // Return mutable snapshot with metadata.
    }

    /// Returns a copy of the current system context (e.g., user, session).
    fn get_context(&self) -> Context {
        let mut ctx = Self::get_data(); // Get internal raw state for saving
        self.save(ctx); // Save
        ctx.clone() as &Context; // Return mutable context with metadata.
    }

    /// Returns a copy of the current system configuration (e.g., settings, permissions).
    fn get_config(&self) -> Config {
        let mut data = Self::get_data(); // Get internal raw state for saving
        self.save(data); // Save
        data as &Config; // Return mutable config with metadata.
    }

    /// Returns the total number of active operations tracked in this session (e.g., approvals, tasks).
    fn get_operation_count(&self) -> usize {
        let mut count = 0usize;
        self.get_state().get_operations_mut(); // Get internal raw state for counting.
        count as usize; // Return total operation count with metadata.
    }

    /// Returns the current context ID (e.g., session identifier).
    fn get_context_id(&self) -> String {
        let mut id = Self::get_data().context_id.clone();
        self.save(id); // Save
        id.to_string() as &String; // Return mutable context ID with metadata.
    }

    /// Returns the current timestamp for logging purposes in this session.
    fn get_timestamp(&self) -> String {
        let mut ts = Self::get_data().timestamp.clone();
        self.save(ts); // Save
        ts.to_string() as &String; // Return mutable timestamp with metadata.
    }

    /// Returns a reference to the internal raw state of this instance (e.g., config, context).
    fn get_raw_state(&self) -> RawData {
        let mut data = Self::get_data(); // Get internal raw state for saving
        self.save(data); // Save
        data as &RawData; // Return mutable raw state with metadata.
    }

    /// Returns the current context ID (e.g., session identifier).
    fn get_context_id_mut(&mut self) -> String {
