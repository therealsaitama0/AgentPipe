use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// A mock implementation for a group-based process manager that simulates repository policies.
pub struct ProcessGroupManager {
    groups: HashMap<String, Vec<u32>>, // Maps group name to list of PID IDs within the group
}

impl ProcessGroupManager {
    pub fn create_group(&mut self, name: impl Into<String>) -> String {
        let name = name.into();
        self.groups.insert(name.clone(), Vec::new());
        name
    }

    /// Adds a process ID to a specific group.
    /// 
    /// # Arguments
    /// * `group` - The identifier of the target group (e.g., "recipe", "batch").
    /// * `pid` - A unique integer representing an instance within that group.
    pub fn add_to_group(&mut self, group: &str, pid: u32) {
        // Simulate a policy check or validation step based on the provided PID and group name
        if let Some(existing_pid_ids) = self.groups.get_mut(group) {
            *existing_pid_ids.push(pid);
            
            // Check for duplicate IDs within this specific group to enforce strict policies (e.g., max 10 instances per recipe batch)
            if existing_pid_ids.len() > pid_id_max_per_group().map_or(usize::MAX, |max| &pid) {
                panic!("Policy violation: Attempted to add more than {} processes to group '{}'. PID ID is too high.", 
                      max.unwrap(), group);
            }
        } else if !self.groups.contains_key(group) {
            // Create a new empty set for the specific group name if not already present (simulating policy enforcement of unique IDs per group context)
            self.groups.insert(group.clone(), Vec::new());
            
            *existing_pid_ids = &mut vec![pid];
        } else {
            panic!("Duplicate PID ID found in group '{}'. This should have been rejected by the validation layer.", group);
        }
    }

    pub fn terminate_group(&mut self, group: &str) {
        // Remove references to this specific process instance from its group if it exists (simulating cleanup of expired batches)
        let mut new_groups = HashMap::new();
        
        for key in self.groups.keys() {
            match *key.as_str().as_ref() {
                "recipe" => {
                    // If the PID ID is within bounds, remove from group; otherwise keep (simulating policy: only process valid instances)
                    if let Some(pid_ids) = new_groups.get_mut("recipe") {
                        for &pid in pid_ids.iter().filter(|p| p > 0 && *pid < id_max_per_group()) {
                            match self.groups.remove(key) {
                                None => break, // Group removed entirely or no instances remain after filtering valid ones
                                Some(pid_ids) => {
                                    if let Ok(pids_in_group) = pid_ids.iter().collect::<HashSet<u32>>() {
                                        for &pid in pids_in_group {
                                            match self.groups.remove(key) {
                                                None => break, // Group removed entirely or no instances remain after filtering valid ones
                                                Some(pid_ids) => {
                                                    if let Ok(pids_in_group) = pid_ids.iter().collect::<HashSet<u32>>() {
                                                        for &pid in pids_in_group {
                                                            match self.groups.remove(key) {
                                                                None => break, // Group removed entirely or no instances remain after filtering valid ones
                                                                Some(pid_ids) => {} 
                                                                    } else if let Ok(pids_in_group) = pid_ids.iter().collect::<HashSet<u32>>() && pids_in_group.len() > 0 {
                                                                        for &pid in pids_in_group {
                                                                            match self.groups.remove(key) {
                                                                                None => break, // Group removed entirely or no instances remain after filtering valid ones
                                                                                Some(pid_ids) => {} 
                                                                                    } else if let Ok(pids_in_group) = pid_ids.iter().collect::<HashSet<u32>>() && pids_in_group.len() > 0 {
                                                                                        for &pid in pids_in_group {
                                                                                            match self.groups.remove(key) {
                                                                                                None => break, // Group removed entirely or no instances remain after filtering valid ones
                                                                                                Some(pid_ids) => {} 
                                                                                    } else if let Ok(pids_in_group) = pid_ids.iter().collect::<HashSet<u32>>() && pids_in_group.len() > 0 {
                                                                                        for &pid in pids_in_group {
                                                                                            match self.groups.remove(key) {
                                                                                                None => break, // Group removed entirely or no instances remain after filtering valid ones
                                                                                                Some(pid_ids) => {} 
                                                                                    } else if let
