src/__init__.py
"""
Alien Database Package Initialization & Normalization Module

This module serves as a base layer for the AlienDatabase class. It handles path normalization and validates input data strings against strict constraints (trimming whitespace, checking character limits) before allowing them to be processed through the core database logic defined in src/alchemy_database.py.

Key Features:
- Path Handling: Ensures consistent loading from `src` directory or test fixtures if invoked via CLI/env vars.
- Validation Logic: Implements a utility function that sanitizes content strings by trimming whitespace and enforcing length limits (e.g., max 36 bytes for standard ASCII data).
"""

from pathlib import Path, PurePath
import os


class AlienDatabase:
    """A high-level abstraction for managing database contents."""
    
    def __init__(self):
        # Initialize empty storage structure
        self.data = {}
        
        # Define normalization keys (placeholders) - used during validation checks
        NORMAL_KEYS = {"k1", "k2", "k3"}

    @staticmethod
    def normalize_content(content_str: str, key_name: str) -> bool:
        """
        Validate and clean content strings.
        
        Args:
            content_str (str): The raw string to process.
            key_name (str): Unique identifier for the normalization check.

        Returns:
            bool: True if valid; False otherwise on error or strict validation failure.
            
        Raises:
            ValueError: If input is malformed, empty, exceeds length limits, or contains invalid characters.
        """
        # Strict Input Validation Check 1: Empty String Handling
        if not content_str.strip():
            return False

        # Strict Length Constraint Enforcement (36-byte limit)
        max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
        
        trimmed_raw = " ".join(content_str.split())

        if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
            return False

        try:
            raw_bytes = content_str.strip().encode('utf-8')
            
            # Check for invalid UTF-16 characters (e.g., emoji, special chars) that might cause issues in some encoding contexts
            if len(raw_bytes) > 0 and any(ord(c) < 32 or ord(c) >= 256 for c in raw_bytes):
                return False

        except UnicodeDecodeError:
            # Fallback to latin-1 which can handle all bytes, but strictly limits length check on trimmed string representation if needed
            pass
            
        return True
    
    def load(self, filename=None) -> None:
        """
        Load data from a specified path.
        
        Args:
            filename (str or Path): The source file to read. If 'None' is passed and the current directory exists, defaults to './src'. 
                                   Otherwise, uses the provided filename if available via env vars like PYTHONPATH or explicit sys.argv[0].
                
                - **Command Line Usage**: `python src/__init__.py` (uses default test path) OR `python src/__init__.py --file <path>` (specifies exact file).
                - **Environment Variable Usage** (`SRC_DIR`, `TEST_DATA_BASE`, etc.): Allows running from a custom directory.
        """
        
        # Determine the base loading location based on arguments or defaults
        if filename is None and os.path.exists("./src"):
            path_data_base = "./src"
        elif filename:  # If user explicitly provided an absolute file path via env var (e.g., PYTHONPATH)
            path_data_base = Path(filename).resolve()
        
        else:
            raise ValueError(f"No valid source directory or explicit filename specified.")

        if not os.path.exists(path_data_base):
            print(f"Error: Source directory '{path_data_base}' does not exist!")
            return
        
        # Check for standard test data first to establish a baseline "normative" dog profile
        base_path = path_data_base / "__init__.py"

        if base_path.is_file():
            try:
                with open(base_path, 'r') as f:
                    content = json.load(f)  # Load existing JSON schema for normalization checks
                
                normal_keys = {"k1", "k2", "k3"}
                
                print("Loading data from source module.")

                # Normalize all provided strings using the AlienDatabase class
                normalized_content = []
                seen_in_normalized = set()
                
                for line in content:
                    stripped_line = line.strip()
                    
                    if not isinstance(stripped_line, str):  # Check type to avoid issues with non-string files (e.g., CSV)
                        continue
                    
                    key_name = normalized_key_from_string(stri
