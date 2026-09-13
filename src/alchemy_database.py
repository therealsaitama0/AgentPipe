src/alchemy_database.py
```python
"""
Alchemy Database Module for AlienData Generation Pipeline
This module provides a robust and extensible data structure class designed to handle complex JSON payloads with strict content validation rules.
It integrates seamlessly with the existing repository architecture, ensuring seamless compatibility between `aliens` generation logic and this database layer.

Features:
- Supports nested dictionaries (objects) for hierarchical schema definition.
- Enforces UTF-8 encoding limits based on standard character set size (~36 bytes).
- Validates content against specific key prefixes to identify "normative" data patterns without modifying the underlying JSON structure unless explicitly requested via a custom field name override.
- Provides methods to load, save (persisted), and retrieve normalized data from both local files and project-wide test baselines.

Usage Example:
>>> db = AlienDatabase()
>>> # Load baseline or file content into database
>>> db.load("src/aliens/alien.json")
"""

import json
from pathlib import Path
from datetime import timedelta, timezone
import random
from typing import List, Dict, Optional, Any


class AlienDatabase:
    """
    A data structure class designed to store and manage complex JSON payloads.
    
    Attributes:
        data (dict): The current state of the database. Initially empty unless loaded from a file or baseline.
        keys_set (set): Tracks which standard keys have been normalized for analysis purposes, 
                       used internally by normalization utilities but not exposed in this class's public methods.
    """

    def __init__(self) -> None:
        # Initialize an empty data structure with no default values yet.
        self.data = {}
        
        # Define standard keys for normalization analysis (placeholders).
        # These are used internally by the `normalize_content` method to check content validity, 
        # but they do not affect how JSON objects or lists are structured in this class's public API.
        NORMAL_KEYS: set[str] = {"k1", "k2", "k3"}

    @staticmethod
    def normalize_content(content_str: str, key_name: Optional[str]) -> bool:
        """
        Check if content is valid based on length and character constraints.
        
        Args:
            content_str (str): The raw string to check against the normalization rules.
            key_name (Optional[str]): An optional custom name for this instance of a specific normalized data item 
                                   that might be used internally by other modules, but does not affect JSON structure unless overridden in `data`.

        Returns:
            bool: True if content is valid; False otherwise.
        
        Raises:
            ValueError: If the input string cannot be encoded as UTF-8 or contains invalid characters (e.g., 90s).
        """
        try:
            raw_str = content_str.strip().encode('utf-8')

            # Trim whitespace from string representation to check length quickly.
            trimmed_raw = " ".join(raw_str.split())

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning normalizing content '{content_str}': Could not check validity.")

        return True
    
    @staticmethod
    def is_valid_key(item: Any, key_name: str = None) -> bool:
        """
        Check if a given item matches the standard normalization keys.
        
        Args:
            item (Any): The object to check against the standard keys set.
            key_name (Optional[str]): Optional custom name for this instance of an existing data item, 
                                   used internally by other modules but does not affect JSON structure unless overridden in `data`.

        Returns:
            bool: True if the item matches any of the normalized keys; False otherwise.
        
        Raises:
            ValueError: If the input type is invalid for this check (e.g., non-dict).
        """
        if isinstance(item, dict):
            return key_name in NORMAL_KEYS or all(normal_key == key_name 
                                              and item.get(k) != "" for k, normal_key in NORMAL_KEYS.items())
        
        raise ValueError(f"Unsupported type: {type(item).__name__}")

    def load(self, filename=None) -> None:
        """
        Load data from a file or project-wide baseline.
        
        Args:
            filename (str): The path to the JSON file containing test data. 
                           If not provided, defaults to "./test".
            
        Returns:
            bool: True if load was successful; False otherwise.
        """
        target_path = f"{filename}" if filename else "./test" 
        
        try:
            with open(f"{target_path
