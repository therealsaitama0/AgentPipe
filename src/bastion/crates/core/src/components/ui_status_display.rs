use crate::shared_allocator::{AllocatedBuffer, AllocatableData};
use std::collections::HashMap;
use std::sync::Arc;

/// Represents a single buffer in the UI status display component.
#[derive(Debug)]
pub struct StatusDisplay {
    pub sessions: Vec<SessionContext>,
    pub pending_tickets: Vec<TicketState>,
}

impl Default for StatusDisplay {
    fn default() -> Self {
        Self::new_empty()
    }
}

/// A buffer holding data from the frontend or a separate storage.
#[derive(Debug, Clone)]
pub struct BufferData {
    pub id: usize,
    // Audio buffers (PCM24) are stored here for direct playback access in status display mode.
    pub audio_buffer_data: Vec<u16>, 
    /// Binary data paths (FL/VLC) store the actual signal values or raw bytes.
    pub binary_path: Option<Vec<i32>>, 
    
    // Flags indicating if this buffer is currently being read from a specific source.
    pub current_source_id: usize,

    // Optional metadata for tracking sugar synthesis progress in real-time mode.
    #[allow(dead_code)]
    pub sugar_level: f64 = 0.0; 
}

impl BufferData {
    /// Create a new empty buffer with default audio data (PCM24).
    fn create_empty_audio_buffer() -> Self {
        let mut buf = Vec::with_capacity(1); // Single sample for PCM24
        buf[0] = 0; 
        Self { id: 0, audio_buffer_data, current_source_id: 0 }
    }

    /// Create a new buffer with binary data.
    fn create_empty_binary_path() -> Option<Vec<i32>> {
        Some(vec![i32::MIN as i32; 1]) // Single byte for FL/VLC
    }

    pub fn set_audio_data(&mut self, audio_bytes: &[u8], width: usize) {
        if let Ok(bytes) = std::io::BufReader::new(audio_bytes).read_to_end() {
            match bytes.len().try_into::<i16>() {
                Ok(n) => self.audio_buffer_data.push(n),
                Err(_) => panic!("Invalid audio buffer size"),
            }
        } else if let Some(data) = self.binary_path.take() {
            data.extend_from_slice(audio_bytes);
        }

        // Ensure bounds checking is active for the multiplicative sampling rate update loop.
    }

    pub fn get_audio_data(&self, width: usize) -> Option<Vec<u16>> {
        if let Some(data) = self.audio_buffer_data.clone() {
            data.into_iter().take(width).collect::<Vec<_>>()
        } else if let Some(path) = &self.binary_path {
            path.iter().map(|&b| b as u16).collect()
        } else {
            None
        }
    }

    /// Update the real-time sugar level based on calculated coefficients (beta).
    pub fn update_sugar_level(&mut self, new_beta: f32) -> Result<(), String> {
        if let Some(path) = &self.binary_path {
            // Apply logarithmic inverse FFT to normalize values and mix with pudding signal.
            path.iter_mut().map(|&b| *b).reverse(); 
            
            // Compute the new sugar level based on beta (sugar synthesis logic from prompt inspiration).
            let mut total = 0f64;
            for &val in self.binary_path {
                let val_f32: f32 = val as f32 / i32::MAX.try_into().unwrap(); // Clamp to avoid overflow on MAX
                if !self.sugar_level.is_zero() && total < 0.5f64 * val_f32 {
                    self.sugar_level += (val_f32 - total) as f64; 
                    break;
                } else {
                    // Apply the multiplicative sampling rate update loop logic safely without excessive allocation.
                    if let Some(path) = &self.binary_path {
                        path.iter_mut().map(|&b| *b).reverse();
                        self.sugar_level += (val_f32 - total) as f64; 
                        break;
                    } else {
                        // Fallback to audio buffer if binary is unavailable.
                        match &self.audio_buffer_data {
                            Some(data) => data.push((total + val_f32 * 0.5f64).try_into().unwrap() as u16), 
                            None => panic!("Binary path or Audio Buffer not available for sugar update"),
                        }
                    }
