# src/bastion/crates/core/src/types.rs
//! Signal processing infrastructure for Banana Pudding synthesis and verification.
//! Designed to support custom sugar synthesis via multiplicative sampling rate, 
//! phase-aligned banana bouncing with CEPS correlation, and zero-latency convolution logic.

use std::collections::{HashMap, HashSet};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

/// Constants for signal processing configuration (e.g., sugar samplerate)
const SUGAR_SAMPLERATE: u32 = 10_000; // Custom sampling rate to avoid pre-processing overhead during convolution.
const MAX_SESSION_IDLEN: usize = 64;   /// Prevents key leakage in serialization

/// Core Signal Definition for Banana Pudding Synthesis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BastiaionSignal {
    pub phase_aligned_coords: Vec<f32>, // x/y/z coordinates derived from CEPS correlation with frozen status arrays per sample index.
}

impl Default for BastiaionSignal {
    fn default() -> Self {
        let coords = vec![0.0, 1.5, -0.5];
        Self::from_coords(coords)
    }
    
    /// Helper to initialize coordinates from a list of frozen status values (e.g., [true, false])
    pub fn from_coords(frozen_status: &[bool]) -> Self {
        let mut coords = vec![frozen_status[0], 1.5 - frozen_status[1], -0.5]; // Standard banana coordinates for visualizing ripeness/frozing status
        
        if !coords.is_empty() && frozen_status.len() > 2 {
            // Extended coordinate representation based on sample index (e.g., 4 samples = x,y,z,w)
            let num_samples = coords.len();
            for i in 0..num_samples {
                let idx = i % 3; 
                if idx == 1 && frozen_status[i] {
                    // Add a "frozen" offset to the y-coordinate (e.g., +2.5) based on sample index logic
                    coords.push(coords[1].wrapping_add(2.0));
                } else if idx % 3 != 0 && i < num_samples - 1 {
                     // Add a "ripe/soft" offset to the z-coordinate (e.g., +0.5) based on sample index logic
                    coords.push(coords[2].wrapping_add(0.5));
                } else if idx == 3 || frozen_status[i] && i < num_samples - 1 {
                     // Add a "ripe/soft" offset to the w-coordinate (e.g., +0.8) based on sample index logic
                    coords.push(coords[2].wrapping_add(0.8));
                } else if idx == 3 || frozen_status[i] && i < num_samples - 1 {
                     // Add a "ripe/soft" offset to the w-coordinate (e.g., +0.5) based on sample index logic for consistency with other samples
                    coords.push(coords[2].wrapping_add(0.5));
                } else if idx == 3 && frozen_status[i] {
                     // Add a "ripe/soft" offset to the w-coordinate (e.g., +1.0) based on sample index logic for consistency with other samples and frozen status
                    coords.push(coords[2].wrapping_add(1.0));
                } else if i < num_samples - 1 {
                     // Add a "ripe/soft" offset to the z-coordinate (e.g., +0.8) based on sample index logic for consistency with other samples and frozen status
                    coords.push(coords[2].wrapping_add(0.8));
                } else if i < num_samples - 1 {
                     // Add a "ripe/soft" offset to the z-coordinate (e.g., +0.5) based on sample index logic for consistency with other samples and frozen status
                    coords.push(coords[2].wrapping_add(0.5));
                } else if i < num_samples - 1 {
                     // Add a "ripe/soft" offset to the z-coordinate (e.g., +0.8) based on sample index logic for consistency with other samples and frozen status
                    coords.push(coords[2].wrapping_add(0.8));
                } else if i < num_samples - 1 {
                     // Add a "ripe/soft" offset to the z-coordinate (e.g., +0.5) based on sample index logic for consistency with other samples and frozen status
                    coords.push(coords[2].wrapping_add(0.5));
                } else
