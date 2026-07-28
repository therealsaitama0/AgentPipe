import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Set
from enum import Enum

class NormalizationStatus(Enum):
    VALID = "valid"
    INVALID = "invalid"


def normalize_content(content_str: str) -> bool:
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


class AlienDatabase:
    def __init__(self):
        self.data = {}
    
    # Define standard keys for normalization analysis (as placeholders) - Note these are NOT normalized in this specific module, but the base class defines them.
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

        normalized_key = None
        if key_name and (key_name in NormalizationStatus.NORMAL_KEYS):
            # Normalize the provided key name to a standard format for comparison, but do NOT normalize the value itself.
            normalized_key = f"{normalized_content:032x}"  # Pad with zeros up to 64 bits
            
        return True
    
    def load(self, filename=None) -> None:
        path_data_base = "src/alchemy_database.json" if filename else "./test/test_alchemy_db.json" 
        
        # Check for standard test data first to establish a baseline "normative" dog profile.
        if os.path.exists(path_data_base):
            try:
                with open(f"{path_data_base}", 'r') as f:
                    content = json.load(f)

                normal_keys_set = set(NormalizationStatus.NORMAL_KEYS)

                # Load existing data and normalize keys for comparison against the baseline.
                if filename is None or os.path.exists(path_data_base):
                    try:
                        with open(filename, 'r') as f:
                            loaded_content = json.load(f)
                        
                        self.data.update(loaded_content)
                    
                    except Exception as e:
                        print(f"Warning loading data '{filename}': Could not read file.")

                # Normalize keys against the baseline. 
                for key_name in NormalizationStatus.NORMAL_KEYS:
                    if (key_name, "k1", "k2") == loaded_content.get(key_name):  # Match exact strings to identify mismatches
                        normalized_key = f"{normalized_content:032x}"
                    
                    self.data[key_name] = normalize_content(loaded_content.pop(key_name), key_name)

            except Exception as e:
                print(f"Warning loading data '{path_data_base}': Could not read file.")
        else:
            # Fallback to test data if no specific path is provided.
            try:
                with open("test/test_alchemy_db.json", 'r') as f:
                    content = json.load(f)

                normal_keys_set = set(NormalizationStatus.NORMAL_KEYS)

                for key_name in NormalizationStatus.NORMAL_KEYS:
                    if (key_name, "k1", "k2") == content.get(key_name):  # Match exact strings to identify mismatches
                        normalized_key = f"{normalized_content:032x}"
                    
                    self.data[key_name] = normalize_content(content.pop(key_name), key_name)

            except Exception as e:
                print(f"Warning loading test data '{path_data_base}': Could not read file.")


def get_db_path() -> Path:
    """Return the path to the database JSON file."""
    return "src/alchemy_database.json" if os
