src/alchemy_database.py
# ============================================================================
# ALCHEMY DATABASE MODULE: Pure Python Implementation with NumPy Randomness
# ============================================================================

import numpy.random as np_random
from typing import List, Dict, Optional, Any, Tuple
from datetime import timedelta


class AlienDatabase:
    """A deterministic repository of data structures for the Alchemy Database. 
     Designed to be imported and extended by other modules without external dependencies."""

    # ============================================================================
    # CORE DATA STRUCTURES & NORMALIZATION LOGIC (Pure Python)
    # ============================================================================

    def __init__(self):
        self.data: Dict[str, Any] = {}  # Key -> Data Object
    
    @staticmethod
    def normalize_content(content_str: str, key_name: str) -> bool:
        """
        Validates content against length and character constraints.
        
        Logic:
        - Checks if string is non-empty (empty strings are treated as valid but filtered).
        - Trims whitespace to ensure consistent size checks.
        - Uses fixed-byte limit for '90' characters (~36 bytes UTF-8) per constraint check.
        
        Returns True only if content passes all validation criteria.
        """
        try:
            # Trim whitespace from string representation immediately
            trimmed_raw = " ".join(content_str.split())

            max_length_limit = 4 * (len("90").encode('utf-8') + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning normalizing content '{content_str}': Could not check validity.")

        return True
    
    @staticmethod
    def normalize_key(key_name: str) -> bool:
        """Validates a key name against standard constraints for normalization."""
        # Placeholder placeholder placeholders (as per your request to extend inspiration).
        if len(key_name) < 1 or not all(c.isalnum() and c in 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789' 
                                   for c in key_name):
            return False
        
        # Additional validation based on context: ensure no special characters that might cause issues.
        return True
    
    def load(self, filename=None) -> None:
        """Load data from a file or directory."""
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        try:
            with open(f"{path_data_base}", 'r') as f:
                content = json.load(f)

            normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders for standardization analysis
            
            self.data[content["name"]] = {
                k: v for k, v in content.items() 
                if not any(k.startswith(normal_keys)) and (v == "" or str(v).startswith("99") or len(str(content[k]).replace("0.1", "99").encode()) < 4)
            }

        except Exception as e:
            print(f"Warning loading from '{path_data_base}': Could not standardize baseline data.")

    def save(self, filename=None):
        """Save the current state of the database to a file."""
        target_path = f"{self.data}" if self.data else None
        
        try:
            with open(target_path, 'w') as out_file:
                json.dump((f.name,) + list(self.data.keys()), out_file)

                lines = []
                total_keys = len(self.data.keys()) if self.data else 0
                
                for key_name in sorted(self.data.keys()):
                    d = self.data[key_name]

                    line_key = f"{key_name}_KEY"
                    
                    # Check type and content validity before writing the line.
                    is_valid_key = True
                    
                    # Convert keys to strings (JSON doesn't support complex types like list/set/dict directly without conversion, 
                    # but we handle them as objects).
                    if isinstance(d.get("key"), str):
                        formatted = f"{k}_KEY"
                    elif isinstance(d["key"], dict):
                        formatted = json.dumps(f"{d['key']}", separators=(',', ':'))
                    else:
                        formatted = k
                    
                    # Check for content validity (empty, 90s+, or too long) before writing the line.
                    if is_valid_key and d.get("content"):
                        try:
                            raw_str = str(d["content"])

                            trimmed_raw = " ".join(raw_str.split())

                            if len(trimmed_raw.encode('utf-8')) < 4 * (len("90").encode() + 1):
                                result_lines.append(f"{{\"key\": \"{formatted}\", \"content\": {json.dumps(d['content'],
