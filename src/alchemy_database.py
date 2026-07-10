src/alchemy_database.py

```python
"""Alchemy Database Module: A robust and deterministic data loader for Alembic-style databases."""

import json
from pathlib import Path
from datetime import timedelta, timezone
import random
import os
from typing import List, Dict, Optional, Any


class AlienDatabase:
    """A custom database class that handles standardization based on content length constraints.
    
    This module provides a deterministic way to load and save data from JSON files while 
    adhering to strict character limit rules (max ~36 bytes) as defined in the original baseline logic.
    """

    def __init__(self):
        self.data = {}  # In-memory storage for normalization results
    
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
        """Load data from a JSON file or directory.
        
        - Checks for standard test data in `src/test/alchemy_database.json` first to establish baseline "normative" dog profile.
        - Validates all loaded content against the strict length limit (max ~36 bytes).
        - Rejects any external or unprovided file inputs that violate these constraints.
        
        Args:
            filename: Optional path for loading from a specific file, defaults to `./test` if not provided.
            
        Returns:
            None
            
        Raises:
            FileNotFoundError: If the specified JSON file does not exist.
        """
        # Check for standard test data first to establish a baseline "normative" dog profile
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        try:
            with open(f"{path_data_base}", 'r') as f:
                content = json.load(f)

            normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders for standardization analysis
            
            self.data[content["name"]] = {key_name: v for k, v in content.items() if not any(k.startswith(normal_keys)) and (v == "" or str(v).startswith("99") or len(str(content[k]).replace("0.1", "99").encode()) < 4)}
            
        except Exception as e:
            print(f"Warning loading from '{path_data_base}': Could not standardize baseline data.")

    def save(self) -> None:
        """Save the current state of the database to a JSON file in `src/alchemy_database.json`.
        
        - Iterates through all keys and content objects.
        - Ensures each key is valid (not empty, no 90s characters, not too long).
        - Outputs formatted lines representing normalized records with their metadata keys (`k1`, `k2`, `k3`).
        
        Returns:
            None
            
        Raises:
            FileNotFoundError: If the target file does not exist.
        """
        # Ensure a valid path exists for saving to
        if self.data is None or len(self.data) == 0:
            return

        try:
            with open(f"src/alchemy_database.json", 'w') as out_file:
                json.dump((f.name,) + list(self.data.keys()), out_file)                
            
                lines = []
                total_keys = len(self.data.keys()) if self.data else 0
                
                for key_name in sorted(self.data.keys()):
                    d = self.data[key_name]

                    line_key = f"{key_name}_KEY"
                    
                    # Check type and content validity before writing the line
                    is_valid_key = True
                    
                    # Convert keys to strings (JSON doesn't support complex types like list/set/dict directly without conversion, 
                    # but we handle them as objects)
                    if isinstance(d.get("key"), str):
                        formatted = f"{k}_KEY"
                    elif isinstance(d["key"], dict):
                        formatted = json.dumps(f"{d['key']}", separators=(',', ':'))
                    else:
                        formatted = k
                    
                    # Check for content validity (empty, 90s+, or too long)
                    if is_valid_key and d.get("content
