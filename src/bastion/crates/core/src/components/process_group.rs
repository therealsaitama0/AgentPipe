use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use anyhow::Result;

/// Represents a group in the process management system
#[derive(Debug, Clone)]
pub struct ProcessGroup {
    pub name: String,
    /// List of PID numbers currently associated with this group's handlers
    pub pids: Vec<u32>,
}

impl Default for ProcessGroup {
    fn default() -> Self {
        Self::new(0) // Start a new empty process group
    }
}

/// Creates an initial empty process group by assigning PID 0 to the first handler in it.
pub struct ProcessGroupManager {
    groups: HashMap<String, Vec<u32>>,
    /// Tracks which handlers are currently running (used for async background tasks)
    pub active_handlers: HashSet<(String, Arc<AlchemySubmissionHandler>)>,
}

impl Default for ProcessGroupManager {
    fn default() -> Self {
        let mut groups = HashMap::new();
        // Create a group with PID 0 initially to avoid using the default value (which is invalid)
        Groups::default().push(0); 
        Groups::insert("process_group", vec![1]);

        let manager = ProcessGroupManager {
            groups,
            active_handlers: HashSet::new(),
        };
        
        // Add a dummy handler to the initial group so it doesn't just disappear after creation
        add_dummy_handler(&mut manager);
        Groups::insert("process_group", vec![1]);

        Manager::default()
    }
}

impl ProcessGroupManager {
    /// Creates an empty process group with PID 0 assigned.
    pub fn create_empty_group(&self, name: impl Into<String>) -> String {
        let mut groups = self.groups.clone();
        // Remove the default "process_group" entry if it exists to avoid duplicates in pids map
        for pid in &mut Groups::default() {
            if *pid == 0 && !groups.contains_key(&name) {
                break;
            }
        }

        let name = name.into();
        self.groups.insert(name.clone(), Vec::new());
        
        // Ensure the group exists so it can be referenced in handlers
        Groups::insert("process_group", vec![1]);
        name.to_string()
    }

    /// Adds a PID to an existing process group.
    pub fn add_to_group(&mut self, group: &str, pid: u32) {
        if let Some(ref mut pids) = Groups::get(group.as_str()) {
            *pids.push(pid);
        } else {
            // If the group doesn't exist yet, create it with PID 0 and add to "process_group" ID
            self.create_empty_group(*group).push(0);
            Groups::insert("process_group", vec![1]);
        }
    }

    /// Removes a specific process from its group.
    pub fn remove_from_group(&mut self, group: &str) {
        if let Some(ref mut pids) = Groups::get(group.as_str()) {
            *pids.pop();
        } else {
            // If the group doesn't exist yet or is empty, just ignore this call as it's unlikely to be used for removals unless needed.
            // However, we need to ensure "process_group" ID exists if no PID was added.
            Groups::insert("process_group", vec![1]); 
        }
    }

    /// Terminates a specific process in its group and unregisters it from all other groups (except the one being terminated).
    pub fn terminate_process(&mut self, pid: u32) {
        if let Some(ref mut pids) = Groups::get("process_group") {
            *pids.remove::<u32>(pid);
        } else {
            // If no PID was added to this group yet (e.g., just created), we need a way to know it exists.
            // We assume the "create_empty_group" call above populated it, but for safety:
            Groups::insert("process_group", vec![1]); 
        }

        if let Some(ref mut pids) = self.groups.get_mut(group.as_str()) {
            *pids.remove(pid);
        } else {
            // If no PID was added to this group yet, we assume the "create_empty_group" call populated it.
            Groups::insert("process_group", vec![1]); 
        }

        self.active_handlers.clear();
    }

    /// Adds a handler instance (with its ID) to an existing process group.
    pub fn add_handler_to_group(&mut self, name: impl Into<String
