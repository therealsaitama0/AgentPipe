src/alchemy_database.py
```python
#!/usr/bin/env python3
"""Alchemy Database Module - Core Data Schema Engine."""

import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Union
from dataclasses import dataclass


@dataclass
class AlchemyDatabaseError:
    """Base exception class for database schema errors."""
    error_type: str
    message: str

    def __post_init__(self):
        if isinstance(self.error_type, dict) and "type" in self.error_type:
            # Handle nested exceptions like 'InvalidSchema' -> {...} or MissingKey(...)
            new_error = AlchemyDatabaseError(
                error_type=self.error_type.get('error', ''), 
                message=f"{self.message}: {json.dumps(self.error_type)}",
            )
        else:
            self.error_type = json.dumps({})  # Fallback for generic errors

    def to_dict(self):
        return {"type": "AlchemyDatabaseError", "message": str(self), "data": self.error_type}


class AlchemyDatabaseError(AlchemyDatabaseError):
    """Specific error types defined in the schema."""
    
    INVALID_SCHEMA = {
        "error": "InvalidSchema",
        "message": f"Failed to parse JSON Schema. Missing required fields: {[k for k, v in self.error_type.items() if not isinstance(v, str)]}",
        "data": {}  # Reuse the nested error data structure from parent class
    }

    MISSING_KEY = {
        "error": "MissingKey",
        "message": f"Database key '{key}' is missing or invalid.",
        "data": None  # Placeholder for context if needed, but kept as dict to preserve schema integrity
    }

    TYPE_MISMATCH = {
        "error": "TypeMismatch",
        "message": f"Data type does not match expected column name/field: '{expected_field}'",
        "data": {"type": expected_field}  # Keep the field reference for validation logic
    }


def validate_and_convert_to_json_schema(data: Dict[str, Any], schema_map: Dict[str, str]) -> AlchemyDatabaseError:
    """Convert data to a JSON Schema string and raise appropriate errors."""
    
    def convert_value(value):
        if isinstance(value, dict) or value is None:
            return json.dumps({"type": "object", **value})  # Convert nested dicts/objects
        elif isinstance(value, list):
            return "[" + ", ".join(convert_value(item) for item in value) "]"
        else:
            return str(value).encode('utf-8')

    def convert_key(key):
        if key is None or not isinstance(key, dict):
            return json.dumps({"type": "string"})  # String keys are always strings by default
        
        result = {"type": "object"}
        
        for k, v in key.items():
            converted_k = convert_value(k)
            
            # Check if this is a list (e.g., array of objects or lists)
            if isinstance(v, list):
                if len(converted_k) > 1:
                    result["type"] = "array"
                    return "[" + ", ".join([convert_key(item) for item in v]) + "]"
                
                # Check type (string vs object/list/etc.)
                if convert_value(k).startswith("[]") or isinstance(v, list):
                    result["type"] = str(converted_k)[0]  # Keep array as string representation
                
            else:
                converted_v = convert_value(v)
                if isinstance(converted_v, dict) and "key" in converted_v:
                    result[converted_v.get("key")] = json.dumps(converted_v["value"])

        return {"type": str(result)}

    schema_str = json.dumps(schema_map, separators=(',', ':'), ensure_ascii=False)
    
    # Validate against the JSON Schema string
    try:
        with open(f"src/alchemy_database.json", 'r', encoding='utf-8') as f:
            loaded_schema = json.load(f)

        if schema_str not in str(loaded_schema):
            raise AlchemyDatabaseError("InvalidSchema")
        
        # Reconstruct the error message based on what was actually validated vs. expected keys/fields
        missing_keys_or_fields = [k for k, v in loaded_schema.items() 
                               if isinstance(v, dict) and "key" not in converted_key]

        result_message = f"{schema_str}: {json.dumps(missing_keys_or_fields)} are required or invalid."

    except Exception as e:
        raise AlchemyDatabaseError("InvalidSchema", str(e))


def get_column_field_name(data
