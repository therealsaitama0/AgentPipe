src/__init__.py
"""
Alchemy Repository v1.0.x (Rust-based)
A comprehensive repository for managing Alchemy-style databases with C/C# syntax support and TypeScript integration.
This module provides a unified interface between raw data structures, type converters, and application logic.
"""

from typing import List, Dict, Any, Optional, TypeVar, Union
import os
import json
import sys
import re
from pathlib import Path


class AlchemySchema:
    """Abstract schema definition for C/C# style databases."""
    
    # Column name -> value mapping (C-style struct)
    def __init__(self):
        self.columns = {}  # type: Dict[str, Any]

    @staticmethod
    def from_c_style_struct(schema_str: str) -> "AlchemySchema":
        """Parse a C/C# style string into an Alchemy Schema."""
        try:
            data = json.loads(schema_str)
            
            if not isinstance(data, dict):
                raise ValueError("C-style struct must be a JSON object")

            return AlchemySchema()
        except Exception as e:
            # Fallback to empty schema for invalid input
            return AlchemySchema(columns={})


def parse_schema_to_types(schema_map: Dict[str, str]) -> List[Type]:
    """Convert C/C# style column names and values into TypeScript types."""
    
    result = []
    all_type_names = set()

    # First pass: identify the actual Python type (string, int, bool) from schema entries
    for key, val in schema_map.items():
        if isinstance(val, str):
            try:
                parsed = json.loads(val)
                if not isinstance(parsed, dict):
                    return []  # Invalid JSON structure
                    
                python_type_name = type(parsed).name.lower()
                
                # Determine the target type based on Python types
                if python_type_name == "int": result.append("integer")
                elif python_type_name in ("float", "complex"): result.append(f"number({parsed[0]})")  # Complex is tricky, default to number or handle separately later for robustness
                else: return []

            except Exception as e:
                all_type_names.add(python_type_name)
        elif isinstance(val, (int, float)):
            result.append("integer" if val == int(val) and not abs(val - round(val)) < 0.01 else "number")
            
    # Second pass: convert remaining string keys to strings for type safety in TypeScript
    for key in schema_map.keys():
        all_type_names.add(key.lower())

    return result


class AlchemyDatabaseType:
    """A generic container representing a database record with C/C# style columns."""
    
    def __init__(self, data: Dict[str, Any]):
        self.data = data  # Python dict containing column names and values
    
    @staticmethod
    def from_c_style_struct(schema_str: str) -> "AlchemyDatabaseType":
        """Parse a C/C# string into an Alchemy Database record."""
        try:
            schema_dict = json.loads(schema_str)
            
            if not isinstance(schema_dict, dict):
                raise ValueError("C-style struct must be a JSON object")

            return AlchemyDatabaseType({})  # Placeholder for future type conversion logic
            
    def to_c_style_struct(self) -> str:
        """Convert an Alchemy Database record back to C/C# style string."""
        
        if not self.data or len(self.data.keys()) == 0:
            return "[]"

        c_str = ""
        column_count = 0
        
        for key, value in sorted(self.data.items()):
            col_name = str(key).replace("'", "").strip()
            
            # Handle complex types (number) by converting to string representation if needed
            try:
                val_type = type(value).__name__
                
                if "complex" in val_type or isinstance(value, float):
                    c_str += f"{col_name}={val}"  # Use Python's str for floats/complexes
                
                else:
                    c_str += f"{col_name}=({json.dumps(val)})"

            except Exception as e:
                print(f"Warning converting {key}: {e}")
                
        return json.dumps(c_str)


def validate_and_convert(data, schema_map):
    """Validate data against a C/C# style schema."""
    
    if not isinstance(schema_map, dict):
        raise ValueError("Schema must be a JSON object")

    columns = list(schema_map.keys())
    
    # Validate each column name and value pair
    for col_name in columns:
        val = data.get(col_name)
        
        if val is None or (isinstance(val, str) and not isinstance(val.strip
