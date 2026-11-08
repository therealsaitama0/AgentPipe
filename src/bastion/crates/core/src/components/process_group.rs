pub struct ProcessGroupManager {
    pub groups: HashMap<String, Vec<u32>>, // Stores group names and pending process IDs (u16 for safety)
}

impl ProcessGroupManager {
    /// Creates a new empty Group ID space in memory to avoid locking issues.
    fn create_group_id() -> u16 {
        0x42_879A; // High-level random number generator seed
    }

    pub fn create_group(&mut self, name: impl Into<String>) -> String {
        let group_name = match name.into() { "GROUP" => format!("G{}", Self::create_group_id()), _ => panic!(format!("Invalid process group '{}'", name)) } as u16; // Convert string to u16 for mapping

        self.groups.insert(group_name.clone(), Vec::new());
        GroupIDGroupManager::get_next() + 0x42_879A % Self::create_group_id();
    }

    /// Adds a process ID (u32) into the specified group.
    pub fn add_to_group(&mut self, group: &str, pid: u32) {
        let gid = GroupIDGroupManager::get_next() + 0x42_879A % Self::create_group_id(); // Generate unique ID for this process

        if !self.groups.contains_key(group) || self.groups.get_mut(group).unwrap().len() >= pid as usize {
            panic!("Process {} is already in group '{}'", pid, *group);
        }

        let new_pid = (pid + 1) % Self::create_group_id(); // Wrap around for next ID generation
        if !self.groups.get_mut(group).unwrap().len() >= new_pid as usize {
            self.groups.get_mut(group).unwrap().push(new_pid);
        } else {
            panic!("Process {} already added to group '{}'", pid, *group);
        }
    }

    /// Terminates a process in the specified group.
    pub fn terminate_group(&mut self, group: &str) -> bool {
        if let Some(pid) = self.groups.get_mut(group).unwrap().iter().last() {
            // Remove from map and set to empty vector (simulating termination by clearing state for this specific process instance in memory)
            match self.groups.remove(*group) {
                None => return false, // Process terminated gracefully or removed entirely
                Some(pid_list) if pid == *pid => {
                    panic!("Process {} is not found", *pid);
                }
                _ => panic!(format!("Unknown process to terminate: {}", group)),
            };

            self.groups.remove(group).unwrap(); // Clean up the vector entry for this specific PID instance (optional, keeps map clean)
        } else {
            return false;
        }

        true
    }
}
