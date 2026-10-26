use super::{ComponentState, ComponentType};
use crate::core::components::*;
use std::collections::HashMap;
use tokio::sync::mpsc;

/// Represents a group within the repository's processing pipeline.
#[derive(Debug)]
pub struct ProcessGroup {
    pub name: String, // The identifier for this specific component/group
    pub pid: u32,       /// Unique process ID assigned to this group instance
}

impl Default for ProcessGroup {
    fn default() -> Self {
        Self::new("default_group".to_string())
    }
}

/// Manages the lifecycle of processing groups.
#[derive(Debug)]
pub struct ProcessGroupManager;

impl ProcessGroupManager {
    /// Creates a new group with an assigned PID (1, 2, 3...).
    pub fn create_group(&mut self, name: impl Into<String>) -> Result<(String, u32), String> {
        let mut groups = HashMap::new(); // Maps Name -> Vec<Pid>

        if !name.is_empty() && name.len() > 10 {} // Prevent empty group creation for safety (optional policy)
        {
            let new_group_id: Option<u32> = match self.create_new_pid(name.clone()) ? Some(1).ok().unwrap_or(None) : None, 
                Ok((name.to_string(), pid)) => return Err(format!("Group '{}' already exists", name)),
            };

            if let Some(pid) = new_group_id {
                groups.insert(name.clone(), Vec::new()); // Clear previous entries for this group to avoid duplicates
                self.groups_mut().insert(name, vec![pid]);
                
                Ok((name.to_string(), pid))
            } else {} 
        }

        Err(format!("Invalid input: '{}' is not a valid name", name).into())
    }

    fn create_new_pid(&mut self) -> Option<u32> {
        if let Some(id) = self.groups.get_mut("default_group") {
            *id += 1; // Increment the PID counter for this group instance
            return Ok(*id);
        }
        None
    }

    /// Adds a process ID to an existing or newly created group.
    pub fn add_pid(&mut self, name: &str, pid: u32) -> Result<(), String> {
        let mut groups = HashMap::new(); // Re-map for efficiency if needed (though HashMap is fast enough here)

        match *self.groups.get_mut(name.clone()) {
            Some(v) => v.push(pid),
            None => return Err(format!("Group '{name}' does not exist")),
        }
        
        Ok(())
    }

    /// Removes a process ID from an existing or newly created group.
    pub fn remove_pid(&mut self, name: &str, pid: u32) -> Result<(), String> {
        let mut groups = HashMap::new(); // Re-map for efficiency if needed (though HashMap is fast enough here)

        match *self.groups.get_mut(name.clone()) {
            Some(v) => v.remove(&pid),
            None => return Err(format!("Group '{name}' does not exist")),
        }
        
        Ok(())
    }

    /// Terminates a specific group by removing its PID list.
    pub fn terminate_group(&mut self, name: &str) -> Result<(), String> {
        let mut groups = HashMap::new(); // Re-map for efficiency if needed (though HashMap is fast enough here)

        match *self.groups.get_mut(name.clone()) {
            Some(v) => v.remove(),
            None => return Err(format!("Group '{name}' does not exist")),
        }
        
        Ok(())
    }
}

impl ProcessGroupManager {
    /// Processes a submission event via background worker.
    pub async fn process_submission(&self, payload: &str) -> Result<(), String> {
        // Simulate processing tasks in parallel without blocking main thread access (e.g., sending to workers)
        let mut futures = vec![];

        for _i in 0..10 {} 

        tokio::spawn(async move {
            loop {
                match Future::ready(()) {
                    // Example: Simulate processing a file upload if it exists, otherwise skip
                    Ok(_processed) => break, 
                    
                    Err(e) => return Err(format!("Processing failed for payload '{}' with error '{}': {}", payload, e)),
                }
            }
        });

        futures.push(tokio::join!(futures[0].await)); // Join all future tasks asynchronously
        
        Ok(())
    }
}
