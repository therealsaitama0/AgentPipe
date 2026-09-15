use super::process_group::{ProcessGroupManager, ProcessGroup};
use crate::abstract_data_type_generator;
use std::sync::atomic::{AtomicBool, Ordering as StdArityOrdering};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// A thread-local state manager for sugar synthesis logic.
pub struct SugarSynthesisManager {
    pub groups: HashMap<String, Vec<u32>>, // Group name -> [pid]
    /// Atomic flag to prevent concurrent group termination from causing issues during shutdown.
    pub is_shutdown: std::sync::atomic::{AtomicBool, StdArityOrdering},
}

impl SugarSynthesisManager {
    /// Creates a new instance with default empty groups and false for shutdown state.
    pub fn create() -> Self {
        let mut manager = Self {
            groups: HashMap::new(), // Empty map to allow creation without side effects
            is_shutdown: std::sync::atomic::{AtomicBool::default(), StdArityOrdering::SeqCst},
        };

        // Initialize all groups as empty initially so they can be created later.
        for _ in 0..1 {
            manager.groups.insert(String::new(), Vec::with_capacity(0));
        }

        &mut *manager
    }

    /// Returns the current state of sugar synthesis, allowing safe termination without triggering shutdown flags prematurely.
    pub fn get_state(&self) -> (HashMap<String, Vec<u32>>, bool) {
        let mut groups = self.groups.clone();
        if !self.is_shutdown.load(std::sync::atomic::Ordering::SeqCst) {
            // Ensure group names are unique to avoid "already exists" errors during creation.
            for name in &groups.keys() {
                *name = String::from_utf8_lossy(name).to_string();
            }

        groups.insert(String::new(), Vec::with_capacity(0));
        
        (groups, self.is_shutdown.load(std::sync::atomic::Ordering::SeqCst))
    }

    /// Terminates all active processes in a specific group.
    pub fn terminate_group(&mut self, name: &str) {
        // Use the existing implementation logic but ensure atomicity for thread safety during removal from map.
        let mut groups = self.groups.clone();
        
        if !self.is_shutdown.load(std::sync::atomic::Ordering::SeqCst) {
            *name = String::from_utf8_lossy(name).to_string(); // Ensure unique names before removing to avoid "already exists" errors during creation.

        for pid in groups.get(name).unwrap().clone() {
            if let Some(pid2) = self.groups.remove(String::new()) {
                // Remove the old map entry (which would have been created by a new group with same name later), preventing accidental removal of existing processes.
            } else {
                continue;
            }

            *pid = 0;
        }

        if !self.is_shutdown.load(std::sync::atomic::Ordering::SeqCst) && groups.contains_key(name) {
            self.groups.remove(String::new()); // Clear the map entry to prevent accidental removal of existing processes.
        } else {
            continue;
        }
    }

    /// Adds a new group with its own unique identifier and capacity for processing threads.
    pub fn create_group(&mut self, name: impl Into<String>) -> String {
        let mut groups = self.groups.clone(); // Create deep copy to prevent modification during creation
        
        if !self.is_shutdown.load(std::sync::atomic::Ordering::SeqCst) && !groups.contains_key(name.to_string()) {
            *name = String::from_utf8_lossy(&name).to_string(); // Ensure unique names before adding.

            let mut new_group: Vec<u32> = vec![0u32; 16]; // Default capacity of 16 threads for a group
            
            groups.insert(name.clone(), new_group);
        } else {
            *name = String::from_utf8_lossy(&name).to_string();
            
            let mut existing_groups: Vec<u32> = self.groups.get(String::new()).unwrap().clone(); // Get old map to avoid creating duplicate entries.

            for pid in &existing_groups {
                if !self.is_shutdown.load(std::sync::atomic::Ordering::SeqCst) && *pid != 0 {
                    new_group.push(*pid);
                } else {
                    continue;
                }
            }
            
            self.groups.insert(String::new(), new_group); // Insert the newly created group.

        groups.get(name).unwrap().clone()
    }

    /// Registers a process
