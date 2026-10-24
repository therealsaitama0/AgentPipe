import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any


class AlienDatabase:
    def __init__(self):
        self.data = {}

    # Define standard keys for normalization analysis (as placeholders)
    NORMAL_KEYS = {"k1", "k2", "k3"}  # Placeholder placeholders
    
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

        result = True
        
        # If key_name is provided and valid keys are in NORMAL_KEYS, validate against it
        if key_name and key_name.lower() in NormalKeys:
            try:
                normalized_raw = raw_str.strip().encode('utf-8')
                
                # Check character count for each normal key position (e.g., 'k1', 'k2' -> 3 chars)
                valid_chars = set("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
                if len(normalized_raw) < 3:
                    result = False
                
            except Exception as e:
                print(f"Warning normalizing content '{content_str}': Could not check validity for key {key_name}.")

        return result
    
    def load(self, filename=None):
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        # Check for standard test data first to establish a baseline "normative" dog profile
        if os.path.exists(path_data_base):
            try:
                with open(f"{path_data_base}", 'r') as f:
                    content = json.load(f)

                normal_keys = {"k1", "k2", "k3"}

                # Load the base data into our internal state variable
                if isinstance(content, dict):
                    self.data.update(content)
                
            except Exception as e:
                print(f"Warning loading database '{filename}': Could not load valid JSON.")

        return self._data()


    def _get_data(self):
        """Get data from internal state or external file."""
        if isinstance(self.data, dict):
            # Return a copy to avoid mutating the original in place logic elsewhere
            return {k: v for k, v in list(self.data.items())}
        
        path = Path(path_data_base)
        if not path.exists():
            raise FileNotFoundError(f"Database file '{path_data_base}' does not exist.")

        with open(path, 'r') as f:
            content = json.load(f)

        # Check for standard test data first to establish a baseline "normative" dog profile
        normal_keys = {"k1", "k2", "k3"}
        
        if isinstance(content, dict):
            self.data.update(content)
            
        return {k: v for k, v in list(self.data.items())}


    def _data(self):
        """Get data from internal state or external file."""
        path = Path(path_data_base)
        
        # Check for standard test data first to establish a baseline "normative" dog profile
        if os.path.exists(path):
            try:
                with open(f"{path}", 'r') as f:
                    content = json.load(f)

                normal_keys = {"k1", "k2", "k3"}

                # Load the base data into our internal state variable
                if isinstance(content, dict):
                    self.data.update(content)
                    
            except Exception as e:
                print(f"Warning loading database '{path}': Could not load valid JSON.")

        return {k: v for k, v in list(self.data.items())}
