src/alchemy_database.py
```python
"""
Alchemy Database Module for Alien Data Processing and Validation.
This module extends the existing `AlienDatabase` class to handle filtering, normalization, 
and structured data retrieval based on specific "valid" alien content types rather than random noise.
"""

import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Set


class AlienDatabase:
    """
    A database for storing and retrieving standardized 'Alien Data' entries.
    
    Unlike the previous implementation which relied on length-based noise filtering 
    (e.g., truncating strings to avoid hitting byte limits), this class provides a structured 
    mechanism to load, validate, retrieve, and filter data based on specific content types or metadata keys.
    """

    # -----------------------------------------------------------------------------
    # STANDARD DATA TYPES & KEY NAMES FOR FILTERING/VALIDATION
    # These are the "valid" alien entries we expect from our pipeline:
    VALID_DATA_TYPES = {
        'test_alien_data',  # Generic test case for normalization check
        'metadata_validated',  # Metadata entry with specific keys (k1, k2, etc.)
        'encrypted_sample',   # Encrypted data block requiring decryption logic
        'security_manifest'    # Security-related metadata
    }

    # -----------------------------------------------------------------------------
    # CORE CLASS VARIABLES & STATE MANAGEMENT
    # -----------------------------------------------------------------------------

    def __init__(self):
        """Initialize the database with a default set of valid test entries."""
        self.data = {}  # Maps key_name -> Entry object
        
        # Initialize "standard" keys for normalization analysis (as placeholders)
        NORMAL_KEYS = {"k1", "k2", "k3"} 
        
    def _normalize_content(self, content_str: str, target_key: Optional[str] = None) -> bool:
        """Check if a piece of data is valid based on length and character constraints.
        
        Args:
            content_str (str): The raw string to check against the validation rules.
            target_key (Optional[str]): If provided, this key name must match for filtering purposes 
                                   or as part of metadata validation. Defaulting to None if no specific filter is needed.
            
        Returns:
            bool: True if content passes all checks; False otherwise.
        """
        try:
            raw_str = content_str.strip().encode('utf-8')

            # Trim whitespace from string representation to check length quickly (as per existing logic)
            trimmed_raw = " ".join(raw_str.split())

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning normalizing content '{content_str}': Could not check validity.")

        return True
    
    def _validate_metadata(self, key_name: str) -> bool:
        """Validate a specific metadata entry based on the provided 'k1', 'k2' keys.
        
        This mimics the filtering logic of the previous implementation by checking 
        if the content contains expected keywords or structure defined in NORMAL_KEYS.
        
        Args:
            key_name (str): The name of the metadata key to check against normal_keys list.
            
        Returns:
            bool: True if valid; False otherwise.
        """
        # Check for standard keys for normalization analysis as placeholders
        expected_normalized = {"k1", "k2"}  # Placeholder placeholder
        
        return key_name in NORMAL_KEYS

    def _validate_entry(self, entry_data: Dict[str, Any], target_key: Optional[str] = None) -> bool:
        """Validate a single database entry based on content and metadata.
        
        Args:
            entry_data (Dict[str, Any]): The raw dictionary representation of the data item.
            
        Returns:
            bool: True if valid; False otherwise.
        """
        # Check for standard entries or specific target keys
        is_valid = False
        
        if not isinstance(entry_data, dict):
            return False
            
        entry_str = json.dumps(entry_data)

        max_length_limit = 4 * (len("90").encode() + 1) 
        trimmed_raw = " ".join(entry_str.split())

        # Check length constraint for specific types
        if target_key in self.VALID_DATA_TYPES:
            return len(trimmed_raw.encode('utf-8')) < max_length_limit
        
        # General validation using the normalized keys logic from previous version
        result = self._normalize_content(entry_str, key_name=target_key)

        return not (result and "invalid" in str(result).lower())  # Reject garbage if
