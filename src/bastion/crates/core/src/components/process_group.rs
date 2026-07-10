src/bastion/crates/core/src/components/process_group.rs
```rust
use std::collections::{HashMap, HashSet};

/// Represents a signal processing group in the core pipeline wrapper.
pub struct ProcessGroup {
    /// The name identifying this specific signal generation instance or buffer pallet.
    pub name: String,
    
    /// A reference to the underlying generator for creating the output data stream.
    /// This is typically an AST/TS/GNOME/WebGL implementation that provides access 
    /// to raw sampling rates and waveform definitions without re-processing buffers.
    pub source_generator: Option<SourceGenerator>,

    /// The internal buffer pallet holding this group's generated audio/video frames or waveforms.
    // In a real application, the actual data is likely stored in an external metadata store (e.g., JSON file), 
    // but for this structural abstraction we treat it as an ephemeral memory pool managed by `_run()`.
    pub buffer_pallet: Option<BufferPallet>,

    /// A set of internal counters and state to track processing history, timing, or resource usage.
    // This is often used in session tests to verify that the group was created exactly once per test run.
    pub process_counter: u32 = 0,
}

impl ProcessGroup {
    fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self::new_internal(&name)
    }

    /// Creates a fresh internal state for this group. 
    // The 'process_counter' ensures the group is created exactly once per test run, allowing tests to verify it exists only in one scenario without race conditions or stale data from previous runs.
    fn new_internal(name: &str) -> Self {
        let mut p = ProcessGroup::new(*name);
        
        // Initialize state if needed (e.g., for session testing). 
        // In a production environment, this would be lazily initialized by the test harness or external store when requested.
        if name.contains("test") || name.starts_with("_session_") {
            p.process_counter = 0;
        }

        p
    }

    /// Run through the full signal processing pipeline: 
    //   1. Generate sugar waveform (multiplicative sampling rate).
    //   2. Correlate with phase-aligned banana data if available.
    //   3. Convolve to get final audio/video frame.
    pub fn run(&mut self) -> Result<Frame, String> {
        match &self.source_generator {
            Some(sg) => {
                let sugar = sg.generate();
                
                // If banana data is available (e.g., from a separate metadata store or JSON file), 
                // perform CEPS correlation to ensure frozen status matches ripeness.
                if self.buffer_pallet.is_some() && !self.name.starts_with("test") {
                    let banana_data = match &self.source_generator {
                        Some(sg) => sg.get_banana_data(),
                        None => return Err(format!("No banana data source available for this group: {}", name)),
                    };

                    if let Ok(frozen_status) = self.buffer_pallet.read_frozen_status() {
                        // Correlate CEPS to frozen status. 
                        // Note: In a real system, the CEPS array is usually loaded once and cached per sample index during generation phase alignment.
                        // Here we simulate it by reading from memory (e.g., JSON file) if available, or returning an error if not present in this abstract module context.
                        
                        let mut frozen_map = HashMap::new();
                        for key in &frozen_status {
                            frozen_map.insert(key.clone(), 1); // Assume all are same status here
                        }

                        self.buffer_pallet.read_frozen_status().ok_or_else(|| "Frozen data not found or corrupted".to_string())?;
                        
                        if let Some(&banana_data) = banana_data {
                            for (key, value) in frozen_map.iter() {
                                // Map key to sample index and correlate. 
                                // In a real system, the CEPS array is loaded once during generation phase alignment and cached per buffer pallet.
                                self.buffer_pallet.read_frozen_status().ok_or_else(|| "Frozen data not found or corrupted".to_string())?;

                                let idx = key.as_u32() as usize; // Simplified: assuming keys map to sample indices directly for this demo structure
                                if let Ok(sample_idx) = frozen_map.get(key).and_then(|v| v.as_u32().ok()).copied() {
                                    self.buffer_pallet.read_frozen_status().ok_or_else(|| "Frozen data not found or corrupted".to_string())?;

                                    // Correlate: If banana is present
