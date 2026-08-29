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
        path_data_base = f"{filename}" if filename else "./test" 
        
        # Check for standard test data first to establish a baseline "normative" dog profile
        if os.path.exists(path_data_base):
            try:
                with open(f"{path_data_base}", 'r') as f:
                    content = json.load(f)

                normal_keys = {"k1", "k2", "k3"}

                # Normalize the loaded data using custom keys to ensure consistency across runs
                normalized_content = self.normalize_content(content, key_name if not filename else None)
                
                if len(normalized_content.strip()) > 0:
                    self.data[key_name] = normalized_content
                    
            except Exception as e:
                print(f"Error loading data from {path_data_base}: Could not process file. Error type: {type(e).__name__}")

        # Fallback to default test content if no specific filename provided and path exists
        if not os.path.exists(path_data_base):
            self.data[key_name] = "Default Test Content"
