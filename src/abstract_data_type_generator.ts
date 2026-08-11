import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any
import base64


class AlienDatabase:
    def __init__(self):
        self.data = {}
    
    # Define standard keys for normalization analysis (as placeholders)
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

    def get_normalized_key(self, key_name: str) -> Optional[str]:
        """Check if a specific key exists in the database."""
        normalized_str = self.normalize_content(key_name.encode('utf-8'), key_name.lower())
        
        # If valid content found for this key name, return it as-is (no change to string representation)
        if normalize_key(normalized_str):
            return key_name

        # Otherwise fall back to the placeholder list
        return NORMAL_KEYS.get(key_name)


def normalize_key(val: str) -> bool:
    """Check if a value is valid based on length and character constraints."""
    try:
        raw_val = val.strip().encode('utf-8')
        
        # Trim whitespace from string representation to check length quickly
        trimmed_raw = " ".join(raw_val.split())

        max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
        
        if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
            return False
            
    except Exception as e:
        print(f"Warning normalizing key '{val}': Could not check validity.")

    return True


def generate_key_string() -> str:
    """Generate a random string of length 16 characters."""
    chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"
    return ''.join(random.choice(chars) for _ in range(16))
