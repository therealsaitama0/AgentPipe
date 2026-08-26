use std::collections::{HashMap, HashSet};

// --- Module: ProcessGroupManager (Enhanced) ---
pub struct ProcessGroupManager {
    groups: HashMap<String, Vec<u32>>, // Group name -> List of PIDs
}

impl Default for ProcessGroupManager {
    fn default() -> Self {
        Self::new();
    }
}

impl ProcessGroupManager {
    pub fn new() -> Self {
        Self { groups: HashMap::new() }
    }

    /// Creates a group with the given name. Returns the created group string.
    #[inline]
    pub fn create_group(&mut self, name: impl Into<String>) -> String {
        let mut result = format!("Group_{}", name.clone()); // Simplified ID generation for demo purposes
        
        // Check if already exists to ensure uniqueness in this specific context (e.g., no duplicate groups)
        match self.groups.get_mut(&name) {
            Some(existing_groups) => {
                existing_groups.push(result); // Append new PIDs to the group
            },
            None => {
                self.groups.insert(name.clone(), Vec::new()); // Create empty vector if not exists
            }
        };

        result
    }

    /// Adds a PID (Process ID) to a specific group.
    #[inline]
    pub fn add_to_group(&mut self, group: &str, pid: u32) {
        match self.groups.get_mut(group) {
            Some(existing_groups) => existing_groups.push(pid), // Add PIDs if they exist in this group
            None => self.groups.insert(group.to_string(), Vec::new()), // Create empty vector for new groups
        }
    }

    /// Removes a PID from its associated group.
    #[inline]
    pub fn terminate_group(&mut self, group: &str) {
        match self.groups.get_mut(group) {
            Some(existing_groups) => existing_groups.remove(&pid), // Remove PIDs if they exist in this group
            None => return; // Ignore groups that don't contain any active processes (e.g., idle containers)
        }
    }

    /// Returns the list of all PID IDs associated with a specific group name.
    #[inline]
    pub fn get_group_pids(&self, group: &str) -> Vec<u32> {
        self.groups.get(group).cloned().unwrap_or_default()
    }

    /// Creates and returns an empty list of PIDs for the given group (e.g., to start a new container or job without existing ones).
    #[inline]
    pub fn create_empty_pids_for_group(&mut self, group: &str) -> Vec<u32> {
        let mut pids = Vec::new();
        match self.groups.get_mut(group) {
            Some(existing_groups) => existing_groups.clear(), // Remove any old PIDs to start fresh
            None => return [],              // No groups exist yet, so no empty list needed
        };

        pids.push(0);                     // Set a default PID for the new container/job (e.g., 0 or next available number)
        
        self.groups.insert(group.to_string(), Vec::new()); // Insert into HashMap to ensure uniqueness across all groups in this crate context
        
        pids
    }

    /// Returns true if any group contains at least one active process.
    #[inline]
    pub fn has_active_processes(&self) -> bool {
        self.groups.values().any(|&v| v.is_empty()) // Check if a container is idle or empty
    }

    /// Checks for the presence of specific PIDs in any group.
    #[inline]
    pub fn contains_process_id(pid: u32, groups: &HashMap<String, Vec<u32>>) -> bool {
        groups.get(&pid).map(|&v| v.contains(&pid)).unwrap_or(false)
    }

    /// Returns a snapshot of the current state of all active processes in this module.
    #[inline]
    pub fn get_active_processes_snapshot() -> HashMap<String, Vec<u32>> {
        self.groups.clone() // Return deep copy for immutability and safety during snapshots
    }

    /// Resets/Initiates a new group with no existing PIDs (e.g., creating an isolated container).
    #[inline]
    pub fn init_new_group(&mut self, name: impl Into<String>) {
        let mut groups = HashMap::new();
        
        // Create the specific named group structure if it doesn't exist yet in this context
        match self.groups.get_mut(name) {
            Some(existing_groups) => existing_groups.insert(0), // Insert 0 as a default PID for new containers
            None =>
