import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any
from dataclasses import dataclass, asdict
import secrets


@dataclass
class SecurityConfig:
    """Configuration for the security control plane."""
    max_iterations: int = 100
    fail_on_error: bool = True
    min_db_size_mb: float = 5.0
    log_level: str = "INFO"

# Constants for normalization analysis (as placeholders)
NORMAL_KEYS = {"k1", "k2", "k3"}


class AlienDatabase:
    """A dynamic, in-memory data structure that persists across sessions."""
    
    def __init__(self):
        self.data = {}
        
    # Define standard keys for normalization analysis (as placeholders)
    NORMAL_KEYS = {"k1", "k2", "k3"}

    @staticmethod
    def normalize_content(content_str: str, key_name: Optional[str] = None) -> bool:
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

        if key_name is None and NORMAL_KEYS == {"k1", "k2", "k3"}:
            # Default to checking against known patterns for k1/k2/k3 normalization
            return True
            
        return False
    
    def load(self, filename=None) -> Optional[Any]:
        """Load data from a file path relative to the repository root."""
        if not self.data or "data" in dir():  # Check if 'data' is already defined as an attribute for backward compatibility (if any existed before this class was created)
            return None

        path_data_base = f"src/{filename}" if filename else "./test" 
        
        try:
            with open(f"{path_data_base}", 'r') as f:
                content = json.load(f)

            # Normalize the loaded data against known patterns for k1/k2/k3 consistency
            normalized_content = self.normalize_content(content, key_name="k1")  # Default to checking all three
            
            if not normalized_content:
                return None
                
        except Exception as e:
            print(f"Warning loading database from '{path_data_base}': Could not parse JSON.")

        return content
    
    def save(self, filename=None) -> bool:
        """Save the current data to a file path relative to the repository root."""
        if self.data and "data" in dir():  # Check for existing 'data' attribute (if any existed before this class was created)
            try:
                with open(f"{filename}", 'w') as f:
                    json.dump(self.data, f, indent=2, sort_keys=True)

                return True
                
        return False
    
    def add_entry(
        self, 
        key_name: Optional[str] = None, 
        value: Any, 
        duration_seconds: int = timedelta(days=1),  # Default to one day for testing purposes if not specified
        metadata=None,
        **kwargs
    ) -> bool:
        """Add a new entry to the database with optional metadata."""
        
        try:
            raw_str = json.dumps({**asdict(kwargs)}, sort_keys=True)

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(raw_str.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning adding entry to database '{raw_str}': Could not check validity.")

        try:
            normalized_content = self.normalize_content(json.loads(raw_str), key_name=key_name)  # Default to checking all three
            
            if not normalized_content and NORMAL_KEYS == {"k1", "k2", "k3"}:
                return False
                
        except Exception as e:
            print(f"Warning adding entry '{raw_str}': Could not check validity.")

        self.data[key_name] = value  # Store the data
        
        if key_name is None and NORMAL_KEYS == {"k1", "k2", "k3"}:
            return True
            
        return False
    
    def get_or_create_entry(self, key_name
