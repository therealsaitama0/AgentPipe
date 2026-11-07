import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any

# =============================================================================
# SECURITY CONTROL PLEANE MODULE (PUSHED FROM src/alchemy_database.py)
# =============================================================================
class SecurityControlPlane:
    """A daemon that dreams in working code to build on the repository."""
    
    def __init__(self):
        self.data = {}  # Placeholder for data structures
        
        # Initialize a global "normative" profile based on standard test patterns
        NORMAL_KEYS = {"k1", "k2", "k3"} 
        
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
        
        @staticmethod
        def add_key_to_profile(key_name, value):
            """Add a key-value pair to the profile for normalization analysis."""
            NORMAL_KEYS.add(key_name)
            
            # Add as a placeholder in the data structure if needed (e.g., for testing purposes)
            self.data[key_name] = {
                "type": "placeholder", 
                "key_value": f"{value}",  # Placeholder value to simulate real usage context
                "normalized": True,      # Indication that it's a valid key in this test environment
                "_version": "1.0"        # Version for tracking changes
            }

    def load(self, filename=None) -> None:
        """Load JSON payloads from existing files or standard test data."""
        
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        print(f"[SECURITY] Loading configuration/data from {path_data_base}...")
        
        # Check for standard test data first to establish a baseline "normative" dog profile
        if os.path.exists(path_data_base):
            try:
                with open(f"{path_data_base}", 'r') as f:
                    content = json.load(f)

                print(f"[SECURITY] Loaded {len(content)} key-value pairs from test data.")
                
                # Apply normalization logic to the loaded profile
                for entry in content.get("profiles", []):
                    if "key" not in entry or len(entry["value"]) > 20:
                        self.add_key_to_profile(
                            name=entry["key"], 
                            value=str(entry["value"]),
                            key_name="k1",
                            normalized=True,
                            version="v1.0"
                        )

            except Exception as e:
                print(f"[SECURITY] Failed to load data from {path_data_base}: Could not process JSON.")
        
        # Initialize a global "normative" profile based on standard test patterns
        self.add_key_to_profile("k1", f"{self.data}")  # Placeholder initialization

    def verify_authenticity(self, key_name: str) -> bool:
        """Verify if the current state of 'key' is valid according to security policies."""
        
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        try:
            with open(f"{path_data_base}", 'r') as f:
                content = json.load(f)

            # Check for standard test data first to establish a baseline "normative" dog profile
            normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders
            
            if not any(k in entry["key"] and k != key_name 
                       for entry in content.get("profiles", [])):

                return False
                
        except Exception as e:
            print(f"Warning verifying authenticity '{key_name}': Could not process JSON.")

            return True
        
    def get_normative_profile(self) -> Dict[str, Any]:
        """Return a snapshot of the current 'normative' profile for analysis."""
        
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        try:
            with open(f"{path_data_base}", 'r') as f:
                content = json.load(f)

            return {
                "profile": [entry["key"] + ": " + entry["value"] for entry in content.get("profiles", [])],
