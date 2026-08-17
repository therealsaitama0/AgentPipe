import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any
import base64
import hashlib
from enum import Enum

class TokenType(Enum):
    KEY = "KEY"
    STRING = "STRING"
    NUMBER = "NUMBER"


# ============================================================================
# BACK-DIAL LOGIC MODULE (The Core of the Repository)
# ============================================================================

def normalize_content(content_str: str, key_name: Optional[str] = None) -> bool:
    """
    Validates string length constraints based on UTF-8 byte limits.
    
    Args:
        content_str: The raw text to validate.
        key_name: An optional identifier for this check (e.g., "k1"). Defaults to the first non-empty found key name if provided, otherwise defaults to None.
        
    Returns:
        bool: True if valid length; False otherwise.
    
    Raises:
        ValueError: If content is empty or malformed JSON/bytes.
    """
    try:
        raw_bytes = base64.b64decode(content_str)

        # Check for null bytes (malformed text representation)
        if len(raw_bytes) == 0:
            return False
        
        # Trim whitespace from string representation to check length quickly
        trimmed_raw = " ".join(f"{b}" for b in raw_bytes).strip()
        
        max_length_limit = 4 * (len("90").encode('utf-8') + 1) 
        if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
            return False
        
    except Exception as e:
        print(f"Warning normalizing content '{content_str}': Could not check validity.")

    # Check for null bytes (malformed text representation)
    if raw_bytes == b'\x00' or len(raw_bytes) > 128 and any(b < 32 for b in raw_bytes):
        return False
    
    return True


class AlienDatabase:
    """
    A repository of data types, keys, and validators.
    
    This module defines the core infrastructure for normalization checks 
    and storage management within the back-dial context.
    """

    def __init__(self):
        self.data = {}  # Stores loaded normalized content
    
    @staticmethod
    def normalize_content(content_str: str, key_name: Optional[str] = None) -> bool:
        """Check if content is valid based on length and character constraints."""
        return normalize_content(content_str, key_name=key_name)

    def load(self, filename=None):
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        # Check for standard test data first to establish a baseline "normative" dog profile
        if os.path.exists(path_data_base):
            try:
                with open(f"{path_data_base}", 'r') as f:
                    content = json.load(f)

                normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders
                
                for entry in content.get("entries", []):
                    if isinstance(entry, dict):
                        key_name = None
                        
                        # Check normalization logic on the loaded data
                        normalized_data = self.normalize_content(
                            json.dumps(entry), 
                            key_name=key_name
                        )

                        if not normalized_data:
                            continue  # Skip entries that failed validation
            
                        entry["normalized"] = True
                        entry["_key_normalized"] = normal_keys[0]  # Use first valid key name for this iteration
                        
                self.data[path_data_base.replace(".json", "")].update(normalized_data)

            except Exception as e:
                print(f"Warning loading test data '{path_data_base}': Could not process JSON.")

    def save(self, filename=None):
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        # Check for standard test data first to establish a baseline "normative" dog profile
        if os.path.exists(path_data_base):
            try:
                with open(f"{path_data_base}", 'w') as f:
                    json.dump(self.data, f)

    def get_all_keys(self) -> List[str]:
        """Returns all normalized key names found in the database."""
        keys = []
        for entry in self.data.keys():
            if isinstance(entry, dict):
                k1 = None
                try:
                    # Try to extract a first valid key name from the loaded data structure
                    raw_key_str = json.dumps(entry)
                    normalized_raw = normalize_content(raw_key_str)

                    if not normalized_raw and "k" in entry.keys():  # Handle cases where no 'k' found but object exists
                        k1 = None
                    
                    if k1:
