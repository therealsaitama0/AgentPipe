src/alchemy_database.py
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

                # Normalize each field in the JSON data against our key namespaces
                for item in content.get('data', []):
                    if isinstance(item, dict):
                        for k, v in item.items():
                            self.normalize_content(str(v), NORMAL_KEYS[k])


if __name__ == "__main__":
    db = AlienDatabase()
    # Mock test data to demonstrate loading and normalization logic
    mock_data = {
        "k1": {"value_abc"},  # Placeholder key: abc (4 chars) - within limit
        "k2": [1, 2],           # Valid string with valid length
        "k3": {},               # Empty dict for reference
    }

    db.load()
    
    print(json.dumps(db.data))
