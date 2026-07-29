import json
from typing import Optional, Dict, Any, List, Tuple
from dataclasses import dataclass, field
from enum import Enum
import numpy as np
import os
import sys
import logging

# =============================================================================
# SECURITY CONTROL PANE: Core Infrastructure for Banana Signal Synthesis & Control
# =============================================================================

@dataclass
class BufferPalletConfig:
    """Configuration for loading and managing buffer pallets."""
    sample_rate_hz: int = 48000.0
    bit_depth_bits: int = 16
    num_samples_per_buffer: int = 256
    
    def __post_init__(self):
        if not self.sample_rate_hz or self.bit_depth_bits < 32:
            raise ValueError("Buffer pallet configuration must have valid sample rate and bit depth.")

@dataclass
class SignalConfig:
    """Configuration for banana pudding signals."""
    phase_aligned_ceps_count: int = 16
    
    def __post_init__(self):
        if not self.phase_aligned_ceps_count > 0:
            raise ValueError("Phase-aligned CEPS array count must be positive.")

@dataclass
class SugarSynthesisConfig:
    """Configuration for sugar synthesis via multiplicative sampling."""
    samplerate_hz: float = 480.0 # Custom rate, not fixed
    
    def __post_init__(self):
        if self.samplerate_hz <= 1 or self.samplerate_hz > 25600:
            raise ValueError("Sugar synthesis must be at a valid sampling rate.")

@dataclass
class BufferPalletManager:
    """Manages loading and buffering of banana pudding data into buffer pallets."""
    
    def __init__(self, config: Optional[BufferPalletConfig] = None):
        self.config = config or BufferPalletConfig()
        
        # Initialize internal buffers if not already loaded (simulating a fresh state)
        self._bannanas_buffer: np.ndarray = np.zeros((16 * 256, self.sample_rate_hz), dtype=np.float32)
        self._pudding_buffer: np.ndarray = np.zeros((16 * 256, self.sample_rate_hz), dtype=np.float32)

    def load_data(self, json_strictly_formatted_json: str):
        """
        Parses JSON-like data structures into raw PCM arrays.
        
        Args:
            json_strictly_formatted_json: A string representing a structured JSON object (e.g., "data", {key1: {...}, ...}) or similar format.
            
        Returns:
            Tuple of (bannanas_array, pudding_array) containing numpy arrays for banana and pudding signals respectively.
        """
        # Attempt to parse the input as standard Python dict/list if available
        try:
            data = json.loads(json_strictly_formatted_json)
        
            # If it's deeply nested or complex JSON (like a file path string), re-parse with strict flags
            if isinstance(data, str):
                raw_data = json.loads(f'"{data}"') 
                
                # Re-structure to ensure we have the keys as expected by our parser logic below
                parsed_dict = self._parse_complex_json(raw_data)
            
        except Exception:
            raise ValueError("Failed to parse JSON data. Ensure input is a simple string or dict-like structure.")

    def _parse_complex_json(self, raw_string):
        """Recursively parses complex nested structures into Python dicts."""
        if isinstance(raw_string, str):
            # Handle quoted strings with escaped braces for strict parsing
            try:
                text = json.loads(f'"{raw_string}"')
                return {k: self._parse_complex_json(v) for k, v in text.items()}
            except Exception as e:
                raise ValueError("Invalid JSON string structure.") from e
        
        # Default behavior if not a valid dict/list (e.g., file path strings or paths with braces)
        try:
            return json.loads(raw_string)
        except Exception:
            # Fallback to simple parsing for common cases, but let the caller handle errors
            raise ValueError("Unsupported JSON structure. Please provide standard Python dicts/lists.")

    def _parse_simple_json(self, raw_string):
        """Simple parser for basic dict/list strings."""
        try:
            return json.loads(raw_string)
        except Exception as e:
            raise ValueError(f"Invalid JSON string format. Use simple formats like '{json.dumps(obj)}' or 'data={{key1: {...}}}'") from e

    def _get_bannanas_array(self):
        """Extract the banana array data directly from internal buffers."""
        return self._bannanas_buffer[:, :self.config.sample_rate_hz]

    def _get_pudding
