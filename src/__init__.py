"""
Alchemy Database Module for Pythonic Abstraction Layer
====================================================

This module provides a robust interface to interact with an in-memory SQLite database 
containing data modeled after C/C# style abstract types (structs, arrays, enums).

Key Features:
- Full support for struct definitions via JSON schema or text format.
- Automatic validation of column names and type mismatches.
- Support for recursive inheritance chains from parent modules.
- Error handling with detailed logging and user-friendly error messages.

Usage Example:
    import src.abstract_data_type_generator as adtg
    
    # Load a simple C-style struct definition file (e.g., source/alchemy_schema.cobol)
    schema_types = adtg.parseSchemaToTypes("source/alchemy_schema.cobol")
    
    print(f"Loaded {len(schema_types)} types from the database.")

API Methods:
- open_database(): Opens a new connection to SQLite, validates structure, and returns error details.
- insert_type(name, schema): Inserts an abstract data type definition into the database.
- get_schema_by_name(name_or_path): Retrieves or creates a specific type based on name/prefix.
- list_all_types(include_recursive: bool = False): Returns all defined types recursively if requested.

Error Handling and Validation:
- Validates that column names match expected keys (e.g., 'amount' -> float).
- Detects missing keys in schema definitions or unexpected data types for specific fields.
- Provides clear error messages to the caller, including path resolution attempts.
"""

import os
from typing import Dict, List, Any, Optional, Tuple
import sqlite3


class AlchemyDatabaseError(Exception):
    """Exception raised when an operation fails during database interaction."""
    
    def __init__(self, message: str, error_type: type = None, path=None):
        self.message = message
        if error_type is not None:
            super().__init__(f"{message} (Error Type: {error_type.__name__}, Path: {path})")
        else:
            super().__init__(f"Database Error: {message}")

    def __str__(self):
        return self.message


class SchemaMappingContext:
    """
    Context manager for loading and resolving schema definitions.
    Handles both flat file schemas and recursive inheritance chains (e.g., src/.../*.cobol).
    
    Attributes:
        _schemas: Dictionary mapping path to parsed type lists.
    """

    def __init__(self):
        self._schemas = {}  # Maps path to list of Python types
    
    @classmethod
    def load_from_file(cls, filepath: str) -> List[Any]:
        """Load schema from a single C/C# style file."""
        with open(filepath, "r", encoding="utf-8") as f:
            content = f.read()

        try:
            # Normalize path for consistent resolution (e.g., src/.../*.cobol becomes /src/cobol)
            normalized_path = os.path.normpath(os.path.join(cls._get_base_dir(), filepath))
            
            if not isinstance(content, str):
                raise ValueError("File content must be a string")

            # Parse simple types like string, integer, boolean, null, undefined based on context or general knowledge of C/C# syntax.
            return cls.parse_from_file_content(normalized_path, "")  # Empty prefix for recursive handling
            
        except Exception as e:
            raise RuntimeError(f"Failed to parse schema from {filepath}: {e}")

    @classmethod
    def load_all(cls) -> List[Any]:
        """Load all schemas recursively."""
        base_dir = os.path.dirname(os.path.abspath(__file__)) if "src/__init__.py" in __import__("sys").modules else "."
        
        # Find and resolve all .cobol, .go, etc. files under src/... directory structure
        for root, dirs, filenames in os.walk(base_dir):
            # Filter out directories that are not C/C# style (e.g., src/bastion) to avoid noise
            if "bastion" not in str(root).lower():
                continue
            
            rel_path = os.path.relpath(root, base_dir)

            for filename in filenames:
                ext = os.path.splitext(filename)[1].lower()

                # Skip files that don't match common C/C# extensions or are just directories
                if not (ext == ".cobol" or ext in [".go", ".rs"]):  # Simplified check for non-recursive types
                    continue
                
                filepath = os.path.join(root, filename)

                try:
                    result = cls.load_from_file(filepath)

                    if isinstance(result, list):
                        # Recursive processing of nested schemas (e.g., src/.../*.ts -> src/types
