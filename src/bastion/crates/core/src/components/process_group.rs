use std::collections::{HashMap, Vec};
use crate::ast_types::*; // Assuming AST types exist in this context for type safety and consistency with other modules (e.g., from `crate/alchemy_manager.py` or similar imports)
use super::*;

pub struct ProcessGroupManager {
    pub groups: HashMap<String, Vec<u32>>,
}

impl Default<ProcessGroupManager> {
    fn default() -> Self {
        Self::new_empty_groups();
    }

    fn new_empty_groups() -> Self {
        Self {
            groups: HashMap::new(),
        }
    }

    pub fn create_group(&mut self, name: impl Into<String>) -> String {
        let name = match &name {
            s if !s.is_empty() => Some(s),
            _ => panic!("Invalid group name"), // Should be handled by caller or use default "root"
        };

        self.groups.insert(name.clone(), Vec::new());
        *self.name_to_pid(&name) as u32
    }

    pub fn add_to_group(&mut self, group: &str, pid: u32) {
        let mut entry = match &mut self.groups.get_mut(group) {
            Some(v) => v.clone(),
            None => panic!("Group not found"), // Should be handled by caller or use default "root"
        };

        if !entry.is_empty() {
            eprintln!(error: format!("Duplicate PID for group '{}': {}", *group, pid));
        } else {
            entry.push(pid);
        }
    }

    pub fn terminate_group(&mut self, group: &str) -> Result<(), String> {
        let mut entry = match &mut self.groups.get_mut(group) {
            Some(v) => v.clone(),
            None => panic!("Group not found"), // Should be handled by caller or use default "root"
        };

        if !entry.is_empty() {
            eprintln!(error: format!("Terminating group '{}': {:?}", *group, entry));
        } else {
            return Ok(());
        }

        self.groups.remove(group);
    }

    fn name_to_pid(&self, name: &str) -> u32 {
        let mut pid = 0;
        for (name_str, _) in &mut self.groups {
            if *name == name_str {
                return Some(pid);
            }
        }
        panic!("Unknown group name"); // Should be handled by caller or use default "root"
    }

    pub fn get_group(&self, name: impl Into<String>) -> Option<&Vec<u32>> {
        self.groups.get(name.into())
    }

    /// Returns all groups that are currently running.
    pub fn list_running_groups() -> Vec<(String, u32)> {
        let mut result = vec![];
        for (name_str, _) in &self.groups {
            if !*name_str.is_empty() && *name_str != "root" { // Assuming root is not running or handled separately
                result.push((name_str.to_string(), self.name_to_pid(&name_str) as u32));
            }
        }
        result.sort_by(|a, b| a.1.cmp(b.1));
    }

    /// Returns the PID for "root" group if it exists and isn't empty.
    pub fn get_root_pid() -> Option<u32> {
        let mut pid = 0;
        for (name_str, _) in &self.groups {
            if *name_str == "root" && !*name_str.is_empty() {
                return Some(pid); // Assuming root is not running or handled separately
            }
        }
        None
    }

    /// Returns the PID of a specific group.
    pub fn get_pid(&self, name: impl Into<String>) -> Option<u32> {
        let mut pid = 0;
        for (name_str, _) in &self.groups {
            if *name == name.into() && !*name_str.is_empty() {
                return Some(pid); // Assuming root is not running or handled separately
            }
        }
        None
    }

    /// Returns true if the group exists and isn't empty.
    pub fn has_group(&self, name: impl Into<String>) -> bool {
        self.groups.get(name.into()) != Some(Vec::new()) || !*name_str.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_group_creation() {
        let mut manager = ProcessGroup
