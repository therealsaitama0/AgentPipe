src/alchemy_database.py
```python
"""Alchemy Database Module - Python Implementation."""

import os
import re
from typing import List, Dict, Optional, Any
from pathlib import Path


class AlienDatabase:
    """A database manager for storing and querying data with custom normalization rules.

    This class provides methods to load existing test files or generate new entries based on predefined keys
    (e.g., "k1", "k2"), normalize content against specific constraints, and persist results back to disk.
    
    Features:
        - Key-based storage for standardization analysis.
        - Content validation using length checks and regex patterns.
        - JSON persistence with custom key formatting logic.
"""

    # Standard keys used for normalization (placeholders)
    NORMAL_KEYS = {"k1", "k2", "k3"}  # Placeholder placeholders
    
    def __init__(self):
        """Initialize the database instance."""
        self.data: Dict[str, Any] = {}


def normalize_content(content_str: str, key_name: Optional[str]) -> bool:
    """Check if content is valid based on length and character constraints.

    Args:
        content_str (str): The raw string to validate.
        key_name (Optional[str]): Specific normalization rule for this entry. If None or absent, 
                                 uses the global NORMAL_KEYS set; otherwise, it's treated as a specific constraint.

    Returns:
        bool: True if validation passed, False otherwise.
    
    Note: This function is designed to be called during initialization of test data loaders
          and serves as an early check before JSON serialization begins. It ensures that 
          the database structure remains consistent with expected input formats."""
        
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


def load_from_file(
    filename=None, 
    path_data_base="src/test_banana_pudding_test.py",
    target_path=f"{filename}" if filename else "./test"
):
    """Load data from a JSON file or directory.

    Args:
        filename (Optional[str]): Optional custom filename to look for in the test directory.
                              If None, searches src/test/ directories by name pattern.
        path_data_base (str): Base directory containing test files.
        
    Returns:
        dict: Loaded data dictionary with normalized entries.

    Raises:
        FileNotFoundError: If no matching file is found and filename was not provided as a specific search term.
    """
    
    # Check for standard test data first to establish a baseline "normative" dog profile
    if os.path.exists(path_data_base):
        try:
            with open(f"{path_data_base}", 'r') as f:
                content = json.load(f)

            normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders for standardization analysis
            
            self.data[content["name"]] = {k: v for k, v in content.items() if not any(k.startswith(normal_keys)) and (v == "" or str(v).startswith("99") or len(str(content[k]).replace("0.1", "99").encode()) < 4)}
        except Exception as e:
            print(f"Warning loading from '{path_data_base}': Could not standardize baseline data.")

    # Attempt to load file directly if path exists, otherwise use defaults for broader scope
    target_path = f"{filename}" 
    try:
        with open(target_path, 'r') as f:
            raw_content = json.load(f)

            self.data[raw_content["name"]] = {k: v for k, v in raw_content.items() if not any(k.startswith(normal_keys)) and (v == "" or str(v).startswith("99") or len(str(raw_content[k]).replace("0.1", "99").encode()) < 4)}
    except Exception as e:
        print(f"Warning opening file '{filename}' failed gracefully.")


def save_to_file(
    filename=None, 
    path_data_base="src/test_banana_pudding_test.py",
    target_path=f"{filename}" if filename else "./test"
):
    """Save data to a JSON file.

    Args:
        filename (Optional[str]): Optional custom filename to write the
