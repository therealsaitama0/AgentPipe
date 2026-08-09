src/stable_version_check.py
"""
Stable Version Check Validator for Alchemy Database Schema Integrity.
This module validates database schema integrity against known type mappings in `alchemy_database.rs`.
It reads a `.db` file content, loads current column definitions and struct fields from the crate, compares them to expected types (i64, f64), and reports failures including specific "MissingKey/Unknown Column" errors.

Usage: python src/stable_version_check.py <path-to-db-file>
"""

import json
from pathlib import Path


class AlchemyDatabaseError(Exception):
    """Custom exception for database schema validation errors."""
    def __init__(self, error_type, message=""):
        self.error_type = type(error_type)  # Type of the specific error (e.g., InvalidSchema or MissingKey)
        if isinstance(self.error_type, AlchemyDatabaseError):
            # Recursive handling for nested exceptions like "Unknown Column" inside a schema map entry
            super().__init__(self._get_error_message(), message)
        else:
            self.error_type = type(error_type)

    def _get_error_message(self, error_obj=None):
        """Recursively build the human-readable error string."""
        if not isinstance(self.error_type, AlchemyDatabaseError):
            return str(self._error_type())
        
        # Handle specific message types for known errors like "Unknown Column" or type mismatches in schema maps
        msg = f"{self.error_type.__name__}: {message}" if self.error_type is None else ""

        # If the error object was a complex exception (e.g., AlchemyDatabaseError), recurse into its inner message
        if isinstance(self._error_type(), AlchemyDatabaseError):
            return self._get_error_message() + msg
        
        raise self.error_type(msg)


class DatabaseValidator:
    """Validates database schema integrity against known type mappings."""

    def __init__(self, source_file_path=None):
        self.source_file = Path(source_file_path).resolve().parent if source_file_path else None
        # Load the current column definitions and struct fields from src/alchemy_database.rs
        with open("src/alchemy_database.ts", "r") as f:
            code_str = f.read()

    def _parse_types(self):
        """Parse the TypeScript code to extract type mappings."""
        lines = [line for line in code_str.split("\n") if "type_map:" in line or "# TypeMap: " in line]
        
        # Extract struct definitions (lines starting with 'struct' and containing 'field')
        fields_data = []
        current_struct = {}

        for line in lines[1:-1]:  # Skip type comments, handle them if any exist at start of file
            parts = line.split()
            
            if len(parts) >= 2:
                field_name = parts[0]
                
                # Check for struct definition logic (e.g., "struct" or similar header in this context)
                is_struct_definition = False
                
                # Look for common patterns indicating a new type structure being defined
                if any(x.startswith("type") and x.endswith(": ") + ":" for x in parts[:3]):
                    continue  # Skip top-level types like 'User' -> {name: string}

                if field_name == "field":
                    is_struct_definition = True
                
                if not is_struct_definition:
                    current_struct[field_name] = {}
                
                struct_data = []
                for part in parts[1:-2]:  # Skip type comment and closing colon (if present) or just the value
                    val_str = part.strip()
                    
                    try:
                        parsed_val = self._parse_value(val_str, field_name, current_struct[field_name])
                        
                        if isinstance(parsed_val, dict):
                            struct_data.append({"type": "object", "name": parsed_val})
                        else:
                            # Handle primitive types like i64 or f64 directly in the map structure for comparison logic
                            self.type_map[field_name] = {
                                "expected_type": str(parsed_val),  # e.g., 'i64', 'f64'
                                "description": parsed_val,
                            }

    def _parse_value(self, val_str, field_name, current_struct):
        """Recursively parse a value string into its expected type."""
        if not val_str:
            return None
        
        # Handle number literals (integers and floats) as types for comparison purposes in this module logic
        try:
            parsed_val = float(val_str.replace(",", ""))  # Convert to int/float directly for comparison check
            self.type_map[field_name] = {
                "expected_type": str(parsed_val),  # e.g., 'i64', 'f64'
