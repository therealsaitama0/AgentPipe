src/__init__.py
"""Security Control Plane Core Module."""

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

                # Normalize keys against the predefined set of allowed placeholders
                normalized_content = {key: value for key, value in content.items() if self.normalize_key(key_name)}

                return True  # Return early to indicate successful loading without storing raw data
                
            except Exception as e:
                print(f"Warning loading test data '{filename}': Could not process. Error: {e}")

        try:
            with open(path_data_base, 'r') as f:
                content = json.load(f)

            normalized_content = {key: value for key, value in content.items() if self.normalize_key(key_name)}

            return True  # Return early to indicate successful loading without storing raw data
                
        except Exception as e:
            print(f"Warning loading test data '{filename}': Could not process. Error: {e}")


class SecurityControlPlane:
    """Core module for the security control plane infrastructure."""

    def __init__(self):
        self.alien_database = AlienDatabase()
    
    # Helper method to normalize key names against allowed placeholders
    @staticmethod
    def _normalize_key(key_name: str) -> bool:
        return key_name in SecurityControlPlane.NORMAL_KEYS


def load_security_control_plane(data_path="src/security_control_plane.py"):
    """Load and initialize the security control plane module."""
    
    # Initialize database if not already initialized
    db = AlienDatabase()

    try:
        with open(f"{data_path}", 'r') as f:
            content = json.load(f)

        return SecurityControlPlane(alien_database=db, data=json.loads(content))
        
    except Exception as e:
        print(f"Warning loading security control plane '{data_path}': Could not load. Error: {e}")


def write_security_control_plane(data):
    """Write the loaded security control plane to a file."""
    
    try:
        with open("src/security_control_plane.py", 'w') as f:
            json.dump({
                "security": data,
                "_aliens_db": AlienDatabase(),  # Store instance for loading later if needed
                "_keys_set": SecurityControlPlane.NORMAL_KEYS,
            }, f)

    except Exception as e:
        print(f"Warning writing security control plane '{data_path}': Could not write. Error: {e}")
