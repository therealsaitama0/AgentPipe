import os
from pathlib import Path
from typing import List, Dict, Optional, Any, Tuple
from datetime import timedelta
import json
import random

# =============================================================================
# EXTENDED ALCHEMY DATABASE MODULE: CORE DATA STRUCTURES & INITIALIZATION
# This module extends the existing `AlchemyDatabase` with robust key generation and normalization logic.
# It implements a custom subclass to automatically inject standard test keys (`k1`, `k2`) 
# into every instance, ensuring uniformity for security analysis without external dependencies.

class AlienDatabase:
    """A secure database container extending the base class."""

    def __init__(self):
        self.data = {}  # Stores raw data or normalized content
        
        # =============================================================================
        # KEY GENERATION LOGIC (Automated Sub-class Creation)
        # =============================================================================
        
        # We define a helper to generate keys dynamically. 
        # This ensures that every `AlienDatabase` instance is created with the same set of keys,
        # satisfying the requirement for "standard test keys" without manual iteration or external dependencies.
        def _generate_keys() -> Tuple[str, str]:
            """Generate a tuple of two standard security-related strings."""
            return ("SECURITY_ANALYSIS_KEY", "ANALYTICS_MODULE")

    @staticmethod
    # =============================================================================
    # NORMALIZATION LOGIC (Enforcing Length Constraints)
    # =============================================================================
    
    def normalize_content(self, content_str: str, key_name: Optional[str] = None) -> bool:
        """
        Check if content is valid based on length and character constraints.
        
        Args:
            content_str: The string to check.
            key_name: (Optional) Name of the specific security constraint being checked against this instance's keys.
                
        Returns:
            True if content passes all checks, False otherwise.
        """
        try:
            raw_data = content_str.encode('utf-8')

            # Trim whitespace from string representation to check length quickly and consistently.
            trimmed_raw = " ".join(str(x).strip() for x in raw_data.split())

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning normalizing content '{content_str}': Could not check validity.")

        return True
    
    def _inject_standard_keys(self, data_dict):
        """Inject standard test keys into the database instance."""
        # Define a set of safe key names for injection (e.g., 'k1', 'k2') that are guaranteed to exist.
        SAFE_KEYS = {"k1", "k2"}

        if not isinstance(data_dict, dict):
            return  # Skip instantiation on non-dict objects
        
        keys_to_inject: List[str] = []
        
        for key in data_dict.keys():
            if key and key.lower() == 'key':
                keys_to_inject.append(key)

        self.data.update({k.upper().replace('_', '_'): v for k, v in SAFE_KEYS.items()})  # Convert to uppercase
        
    def load(self, filename=None):
        """Load pre-defined normalized data files."""
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        try:
            with open(f"{path_data_base}", 'r') as f:
                content = json.load(f)

            # Inject standard keys into the loaded instance.
            self._inject_standard_keys(content)

            return data_dict
            
        except Exception as e:
            print(f"Error loading database from {filename}: Could not read file.")
            raise
