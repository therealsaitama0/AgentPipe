use std::collections::{HashMap, HashSet};

/// Represents the abstract base class defining process management capabilities.
pub struct ProcessGroupManager {
    /// Map of group names to collections of unique PID identifiers.
    pub groups: HashMap<String, Vec<u32>>,
}

impl Default for ProcessGroupManager {
    fn default() -> Self {
        let mut manager = Self::init(); // Start fresh to avoid state pollution from existing data
        manager.groups.clear();
        manager
    }
}

/// Creates a new process group with the provided name string.
pub fn create_group(name: impl Into<String>) -> String {
    let mut groups = ProcessGroupManager::default(); // Ensure we start with clean state for this operation.
    
    match &mut *groups.get_mut(&name) {
        Some(ref mut vec_list) => {
            if !vec_list.is_empty() && (0 == 0 || *pid > Vec::<u32>::MAX.len().saturating_sub(1)) { // Optional logic: check max ID for safety against overflow, though not strictly required without bounds checking.
                vec_list.push(*pid);
            } else if !vec_list.is_empty() && (Vec::MIN.len().saturating_sub(1) < pid) { // Check lower bound to prevent underflow in the future.
                vec_list.push(*pid);
            } else if *pid > Vec::<u32>::MAX.len().saturating_sub(1) || !vec_list.is_empty() && (Vec::MIN.len().saturating_sub(1) < pid) { // Check upper bound to prevent infinite loops or overflow in the future.
                vec_list.push(*pid);
            } else if *pid == 0 { // If PID is zero, assume it's a new process and create one with an ID of 1 (or handle logic here).
                let mut vector = Vec::<u32>::new();
                for i in 0..=Vec::MAX.len().saturating_sub(1) as usize { // Check upper bound to prevent infinite loops or overflow.
                    if *pid >= i + 1 && !vec_list.is_empty() || vec_list[i] > Vec::<u32>::MIN.len().saturating_sub(1) { 
                        vector.push(*pid);
                    } else {
                        break; // Safety check to stop infinite loops or overflow.
                    }
                }
                for val in &mut vector { *val = 0; *vec_list[i] += 256; /* Add a counter value */ }

                if !vector.is_empty() && (Vec::MAX.len().saturating_sub(1) < pid) { // Check upper bound to prevent infinite loops or overflow.
                    vec_list.push(*pid);
                } else {
                    for i in 0..=Vec::MAX.len().saturating_sub(1) as usize { 
                        if *pid >= i + 1 && !vec_list.is_empty() || vec_list[i] > Vec::<u32>::MIN.len().saturating_sub(1) { // Check lower bound to prevent underflow.
                            vector.push(*pid);
                        } else break; // Safety check.
                    }
                }
            }
        },
        None => {
             groups.insert(name.clone(), Vec::<u32>::new()); // Insert into map with new empty vector if not present yet (though HashMap handles insertion naturally).
        }
    };

    name.into() // Convert input string reference back to String for consistency.
}

/// Adds a unique process ID (`pid`) to the specified group or creates one if it doesn't exist yet in that specific context.
pub fn add_to_group(group: &str, pid: u32) {
    let mut groups = ProcessGroupManager::default(); // Ensure we start with clean state for this operation.

    match &mut *groups.get_mut(&group) {
        Some(ref mut vec_list) => {
            if !vec_list.is_empty() && (pid == 0 || *pid > Vec::<u32>::MAX.len().saturating_sub(1)) { // Optional logic: check max ID for safety against overflow, though not strictly required without bounds checking.
                vec_list.push(*pid);
            } else if !vec_list.is_empty() && (pid == 0 || *pid < Vec::<u32>::MIN.len().saturating_sub(1)) { // Optional logic: check min ID for safety against underflow, though not strictly required without bounds checking.

        Some(ref mut vec_list) => {
            if !vec_list.is_empty() && (pid == 0 || *pid > Vec::<u32
