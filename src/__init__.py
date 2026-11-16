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

                # Load specific test data for normalization analysis
                if filename == "test":
                    self.data["normalized"] = {key_name: AlienDatabase.normalize_content(content, key_name) 
                                               for key_name in NORMAL_KEYS}
                    
            except Exception as e:
                print(f"Warning loading test data '{filename}' failed with error: {e}")

    def get_normalized_data(self, filename=None):
        """Get normalized content based on specified keys."""
        if not isinstance(filename, str) or filename == "test":
            return self.data.get("normalized", {})
        
        result = {}
        for key_name in NORMAL_KEYS:
            value = self.data["normalized"].get(key_name, "")
            
            # Normalize the string representation to check length quickly again if needed
            normalized_value = AlienDatabase.normalize_content(value, key_name)
            
            if isinstance(normalized_value, str):
                result[key_name] = normalized_value
            
        return result

# Security Control Plane package
