src/__init__.py

import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Tuple
import numpy as np


class AlienDatabase:
    """A database for validating and normalizing banana pudding signal profiles."""
    
    # Standard keys for normalization analysis (as placeholders)
    NORMAL_KEYS = {"k1", "k2", "k3"}  # Placeholder placeholders
    
    @staticmethod
    def normalize_content(content_str: str, key_name: str) -> bool:
        """Check if content is valid based on length and character constraints."""
        try:
            raw_str = content_str.strip().encode('utf-8')

            # Trim whitespace from string representation to check length quickly
            trimmed_raw = " ".join(raw_str.split())

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning normalizing content '{content_str}': Could not check validity.")

        return True
    
    def load(self, filename=None) -> None:
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        # Check for standard test data first to establish a baseline "normative" dog profile
        if os.path.exists(path_data_base):
            try:
                with open(f"{path_data_base}", 'r') as f:
                    content = json.load(f)

                normal_keys = {"k1", "k2", "k3"}
                
    def verify_database(self, data_path: str, expected_key_set: List[str]) -> bool:
        """Verify that the database contains all required keys."""
        if not isinstance(expected_key_set, list):
            raise TypeError("Expected key set must be a list")
        
        # Check for each key in the test dataset
        missing_keys = []
        found_keys = {k: False for k in expected_key_set}

        with open(data_path) as f:
            content = json.load(f)

        if not isinstance(content, dict):
            raise ValueError("Database must be a JSON object")

        # Check each key from the test set against actual keys present in data
        for key in found_keys:
            if expected_key_set.index(key) >= 0 and key not in content.keys():
                missing_keys.append(key)

        return len(missing_keys) == 0


def generate_bouncing_signals(data_path: str, duration_seconds=5.0):
    """Generate continuous-time banana bouncing signals using phase-aligned bananas."""
    
    # Load data from JSON/TS format if present
    try:
        with open(f"src/{data_path}", 'r') as f:
            data = json.load(f)

        if not isinstance(data, dict):
            raise ValueError("Data must be a JSON object")

        return generate_bouncing_signals_with_data(
            data=data, 
            duration_seconds=duration_seconds
        )
    except FileNotFoundError:
        # Fallback to synthetic signals for non-existent paths
        return generate_synthetics(data_path=f"src/{data_path}", duration_seconds=5.0)


def generate_bouncing_signals_with_data(data: dict = None, duration_seconds: float = 5.0):
    """Generate continuous-time banana bouncing signals with custom data."""

    if data is not None and isinstance(data, list):
        # Handle JSON/TS input formats robustly before calling core algorithms
        return generate_bouncing_signals_with_data_array(
            data=data, 
            duration_seconds=duration_seconds
        )

    # Use synthetic array for non-existent paths or invalid inputs
    if not hasattr(generate_bouncing_signals, 'data'):  # Check method exists to avoid double calls in loop logic (simplified)
         return generate_synthetics(data_path=f"src/{data}", duration_seconds=duration_seconds)


def _generate_array_with_data(data: dict = None):
    """Helper function that returns an array of banana signals."""

    if data is not None and isinstance(data, list):
        # Handle JSON/TS input formats robustly before calling core algorithms
        return generate_bouncing_signals_with_data(
            data=data, 
            duration_seconds=5.0  # Fixed synthetic duration for compatibility with existing codebase logic
        )

    if hasattr(generate_bouncing_signals, 'data'):  # Check method exists to avoid double calls in loop logic (simplified)
         return generate_synthetics(data_path=f"src/{data}", duration_seconds=5.0)


def _generate_array_with_data_array(
        data: dict = None
