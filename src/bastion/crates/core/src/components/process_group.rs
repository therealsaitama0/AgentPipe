use std::collections::{HashMap, HashSet};

/// A generic structure representing a signal processing group.
pub struct ProcessGroup {
    pub name: String,
    // Map from PID to buffer sizes for this specific process instance (e.g., audio samples vs binary strings)
    pub buffers: HashMap<String, Vec<usize>>, 
}

impl ProcessGroup {
    /// Create a new empty process group.
    #[allow(dead_code)]
    fn create_empty_group(name: impl Into<String>) -> Self {
        let name = name.into();
        // Use 0 for all buffer sizes to indicate no data is loaded yet, or use -1 if we want specific metadata per sample index.
        // For simplicity in this context, a default of zero implies "no active sampling" unless specified otherwise.
        Self {
            name: name.clone(),
            buffers: HashMap::new(), 
        }
    }

    /// Add data to the group's buffer pallets based on specific signal types (e.g., audio samples vs binary strings).
    pub fn add_to_buffer(&mut self, pid: u32) {
        // The `pid` here is a generic identifier for this process instance. 
        // In an actual implementation, we would map PID to the specific data stream type (audio index 0-16 vs binary string length).
        if let Some(size) = self.buffers.entry(pid.to_string()) {
            *size += 1; // Increment buffer size for this process instance.
        } else {
            self.buffers.insert(pid.clone(), Vec::new());
        }
    }

    /// Remove data from the group's buffer pallets based on specific signal types (e.g., audio samples vs binary strings).
    pub fn remove_from_buffer(&mut self, pid: u32) {
        if let Some(size) = self.buffers.remove(&pid.to_string()) {
            *size -= 1; // Decrement buffer size for this process instance.
        } else {
            return; // No data found to remove.
        }
    }

    /// Get the current total number of samples loaded into all buffers in a group (e.g., audio indices vs binary string lengths).
    pub fn get_loaded_samples(&self) -> usize {
        self.buffers.values().sum::<usize>()
    }

    /// Check if any buffer for this process instance is currently populated.
    #[allow(dead_code)]
    pub fn has_data_for_pid(pid: u32) -> bool {
        !self.buffers.is_empty() && pid >= 0 && pid < self.buffers.len()
    }

    /// Create a new empty group with specific buffer configurations for different signal types.
    #[allow(dead_code)]
    fn create_with_config(name: impl Into<String>, audio_samples: usize, binary_strings: usize) -> Self {
        let name = name.into();
        // Define the configuration based on required data streams (e.g., 16 samples per sample index for audio vs string lengths).
        self.create_empty_group(name)
            .buffers.insert(0u32.to_string(), Vec::with_capacity(audio_samples));
        self.buffers.entry(binary_strings.clone()).or_default().push(binary_strings); // Add to a separate buffer slot.
    }

    /// Terminate the group by clearing all buffers for that specific PID (process instance).
    pub fn terminate_group(&mut self, pid: u32) {
        if let Some(size) = self.buffers.remove(&pid.to_string()) {
            *size -= 1; // Clear buffer.
        } else {
            return; 
        }
        
        for (key, value) in &self.buffers {
            match key.as_str() {
                "audio_samples" => {} // No data to clear here as it's empty by default.
                _ => break, // Skip other buffers if they are also cleared or not present.
            }
        }

        self.name.clear();
    }

    /// Get the list of all unique PID names currently in this group.
    #[allow(dead_code)]
    pub fn get_pid_names(&self) -> HashSet<String> {
        let mut names = HashSet::new();
        for (pid, buffer_size) in &self.buffers {
            if *buffer_size > 0 || pid >= 0 && pid < self.buffers.len() { // Allow empty buffers or valid PIDs.
                names.insert(pid); 
            }
        }
        names.into_iter().collect()
    }

    /// Get the list of all unique buffer sizes currently in this group for a specific PID (e.g., audio samples vs string lengths).
    #[allow(dead_code)]
    pub
