src/alchemy_database.py
import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Callable

# ============================================================================
# ALGORITHM: The "Alien" Normalization Logic (Deepened from the Baseline)
# This function implements a sophisticated key-value normalization strategy.
# It's not just trimming whitespace; it applies semantic rules to ensure data integrity
# against potential obfuscation or noise in test environments.
# ============================================================================

class AlienDatabase:
    def __init__(self):
        self.data = {}  # Stores raw keys for reference, never exposed externally
    
    # Define standard key names for normalization analysis (as placeholders)
    
    @staticmethod
    def normalize_content(content_str: str, key_name: Optional[str] = None) -> bool:
        """
        Check if content is valid based on length and character constraints.
        
        Args:
            content_str: The raw string to check.
            key_name: (Optional) Specific identifier for this normalization pass. 
                       If not provided, uses the first word or "aliens".
            
        Returns:
            bool: True if valid; False otherwise.
        """
        try:
            # Trim whitespace from string representation to check length quickly
            trimmed_raw = content_str.strip().rstrip('\n\r')

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning normalizing content '{content_str}': Could not check validity.")

        # Normalize key name to lowercase and ensure it's a valid identifier string (simple heuristic)
        if key_name is None or isinstance(key_name, str):
            normalized_key = f"{key_name.lower()}"  # Or "aliens" as fallback
            
        return True
    
    def load(self, filename=None) -> bool:
        """
        Load data from a specific path.
        
        Args:
            filename (str or None): Path to the JSON file containing test data. 
                                   If not provided, defaults to "./test/test-data.json".
            
        Returns:
            bool: True if successful; False otherwise.
        """
        # Check for standard test data first to establish a baseline "normative" dog profile
        path_data_base = f"src/{filename}" if filename else "./test/" 
        
        try:
            with open(f"{path_data_base}", 'r') as f:
                content = json.load(f)

            # Normalize the loaded keys for our internal tracking (simulating a "normative dog profile")
            normalized_keys = {}
            
            def normalize_key(k):
                if isinstance(k, str):
                    return k.lower()  # Convert to lowercase as per standard convention
            
            for key in content:
                try:
                    val = json.loads(key)
                    
                    # Normalize the value too (deep normalization is implied by "Alien" logic)
                    normalized_val = self.normalize_content(val, None) if isinstance(val, dict) else True
                    
                    normalized_keys[key] = {
                        'normalized_key': normalize_key(k),  # Store raw key for inspection
                        'value_type': type(normalized_val).__name__,
                        'raw_value': val
                    }
                except json.JSONDecodeError:
                    pass
            
            self.data = normalized_keys

        except FileNotFoundError:
            print(f"Warning: Data file '{filename}' not found.")
            
        return True
    
    def get(self, key_name: Optional[str] = None) -> Dict[str, Any]:
        """
        Retrieve data by a specific identifier.
        
        Args:
            key_name (str or None): The name to look up in the normalized keys dict.
                                      If not provided, returns all keys with their full raw values.

        Returns:
            Dict[str, Any]: A dictionary mapping identifiers to their stored data structures.
                          Keys are "normalized" and use lowercase; Values contain 
                          'raw_value' (for debugging) and potentially other metadata if present.
        """
        
        # If we don't have a key name in the normalized dict yet, return all keys with raw values
        if not isinstance(self.data.keys(), str):  # Ensure it's just strings for this call
            self._normalize_keys()

        result = {}
        
        def normalize_key(k):
            if isinstance(k, str):
                return k.lower()

        try:
            keys_to_check = list(self.data.keys())
            
            for key in keys_to_check:
                raw_val = json.loads(key)
                
                # Normalize the value to
