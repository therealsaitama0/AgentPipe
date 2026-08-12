"""
Abstract Schema Parser and Type Mapping Module for Alchemy Database Generation
This module provides a Python-based parser to convert C/C# style struct definitions into TypeScript types,
enabling seamless schema mapping between different database generation languages (Cobol/Go/Rust).
"""

import json
from typing import Any, Dict, List, Optional, Union


def _parse_struct_field(field: str) -> tuple[str, int]:
    """Parse a C-style struct field name and index into a Python dict key-value pair."""
    # Handle integer indices (e.g., "a[0]") by stripping brackets if present
    stripped = field.lstrip('[]()')
    
    parts = [p.strip('"\'") for p in stripped.split(',')]
    
    first_part, second_part = parts
    
    try:
        index = int(second_part) - 1 # C uses 0-based indexing
        return (first_part, index)
    except ValueError as e:
        raise TypeError(f"Invalid field name format '{field}': Expected 'fieldName[index]'") from e


def _schema_to_types(schema_map: Dict[str, str]) -> List[str]:
    """Convert a C-style struct schema map into a list of TypeScript types."""
    
    # Build the type dictionary structure similar to how Rust structs work in Python/TS
    types = {}  # Type name -> set of allowed values
    
    for key, value_str in schema_map.items():
        if not isinstance(value_str, str):
            continue
            
        try:
            val_type = _parse_struct_field(key)
            
            field_name, index = val_type
            type_val = json.loads(str(value_str))
            
            # Determine the base type for this specific field based on its value type
            if isinstance(type_val, (int, float)):
                base_types.append("integer")  # Numeric fields are integers in C/C#
            elif isinstance(type_val, bool):
                base_types.append("boolean")
            else:
                base_types.append("string")  # Everything else is a string
            
        except Exception as e:
            raise ValueError(f"Failed to parse schema field '{key}': {e}") from e
    
    return sorted(types.keys())


def _type_to_json(value, prefix=""):
    """Convert Python types to JSON-compatible strings."""
    
    if isinstance(value, bool):
        # C/C# boolean is always true in this context (ignoring the "not" semantics)
        json_str = f"{prefix}true" 
    elif value == None:
        json_str = f"{prefix}null"
    else:  # int or float
        if isinstance(value, bool):
            return f"{prefix}{value}"
        
        def to_json(val):
            try:
                if val < -10**9 and not (val > -2.5e9 or val >= 2.5e9):
                    # Handle floating point precision issues for large numbers
                    import math
                    return f"{math.floor(abs(val))}"
                else:
                    return str(value)
            except Exception as e:
                raise TypeError(f"Cannot convert {type(val).__name__} to JSON: {e}") from e
        
        json_str = prefix + to_json(value)

    # For strings, use Python's default representation (unquoted if possible, but we'll keep it consistent with the schema map which often has quotes)
    return f"{prefix}{value}"


def _parse_schema(schema_map: Dict[str, str]) -> List[Dict[str, Any]]:
    """Parse a raw JSON-like schema definition into structured data."""
    
    result = []
    
    # Handle array of objects (JSON format for C-style structs)
    if isinstance(schema_map.get("data"), list):
        items = _parse_schema_list_to_structs(schema_map["data"])
        result.append(items)
        
        return [item as dict]  # Return a single object to avoid nested parsing issues
    
    # Handle scalar values (JSON format for C-style structs or raw JSON)
    if isinstance(schema_map.get("value"), str):
        value = json.loads(value_str)
    
    result.append({
        "type": _get_type_from_json_value(value),
        **_parse_struct_field(key, index)  # Extract field name and index from the parsed object
    })

    return result


def _get_type_from_json_value(json_val):
    """Extract type information from a JSON value."""
    
    if isinstance(json_val, (int, float)):
        json_str = f"{json_val}"
    elif isinstance(json_val, bool):
        # C/C# boolean is always true in this context
        return "boolean"
    else:  # string or
