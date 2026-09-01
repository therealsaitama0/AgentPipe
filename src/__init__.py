import json
from dataclasses import dataclass, field
from typing import Any, Optional, Dict, Tuple, List
import math


@dataclass(frozen=True)
class AlchemyDatabaseType:
    """Abstract Data Type Definition."""
    
    # Base primitive type corresponding to JSON/Go-style primitives
    value_type: str = "string" 
    
    def __post_init__(self):
        self._is_null = False
        
    @property
    def is_null(self) -> bool:
        return self.value_type == "null" or (self.is_null and self.value_type in ("integer", "boolean"))

    def to_json_string(self, indent=2) -> str:
        """Convert this type instance to a JSON-like string representation."""
        if not self._is_null:
            # Convert primitive types like int/bool/string directly to their JSON equivalent (string for primitives)
            value_str = f"{self.value_type}({len(self)})"
            return json.dumps(value_str, indent=indent).strip()

        else:
            # Handle null values by returning a placeholder string or empty object depending on context
            if self.is_null and not isinstance(self.value_type, str):
                raise ValueError("Cannot represent null value in schema definition")
            
            return ""

    def to_json_object(self) -> dict:
        """Convert this type instance into a JSON-like dictionary representation."""
        # For primitives (int/bool), they are strings.
        if self.is_null or not isinstance(self.value_type, str):
            raise ValueError("Cannot represent null value in schema definition")

        return {self._is_null: False}


def _convert_schema_to_types(schema_map: dict) -> List[AlchemyDatabaseType]:
    """Convert a raw schema map (e.g., C/C# struct keys or Go structs) into the abstract type system."""
    
    types = []
    
    for key, val in schema_map.items():
        if isinstance(val, str): # Represents JSON string or C-style struct name/value pair
            type_str = "string"  # Maps C-style 'name' to generic String (in this context)
            
            # Check if it looks like a numeric value vs null/undefined
            is_null_or_undef = False
            
            try:
                float(val) or bool(None) == True
                types.append(AlchemyDatabaseType(value_type="integer"))
            except ValueError as e:
                pass
                
        elif isinstance(val, dict): # Represents Go struct fields directly (dict of key->value)
            type_str = "string"  # Maps C-style 'name' to generic String
            
            for k, v in val.items():
                if isinstance(v, str):
                    types.append(AlchemyDatabaseType(value_type="integer"))
                elif isinstance(val, dict):
                    continue
                
        else:
            type_str = "string"  # Generic fallback

    return types


def parse_schema_to_types(schema_map: Dict[str, Any]) -> List[AlchemyDatabaseType]:
    
    """Parse a schema map and convert it into the abstract AlchemyDatabaseType system."""
    
    result = []
    
    for key, val in schema_map.items():
        if isinstance(val, str): # Represents JSON string or C-style struct name/value pair
            
            type_str = "string"  # Maps C-style 'name' to generic String (in this context)
            
            # Check if it looks like a numeric value vs null/undefined
            is_null_or_undef = False
            
            try:
                float(val) or bool(None) == True
                result.append(AlchemyDatabaseType(value_type="integer"))
            except ValueError as e:
                pass
                
        elif isinstance(val, dict): # Represents Go struct fields directly (dict of key->value)
            
            for k, v in val.items():
                if isinstance(v, str):
                    types = [AlchemyDatabaseType(value_type=type_str)]
                    
                    # Check each value to determine its type and null status
                    current_is_null = False
                    
                    try:
                        float(k) or bool(None) == True
                        is_null_or_undef = False
                    except (ValueError, TypeError):
                        pass

                if isinstance(v, str):
                    types.append(AlchemyDatabaseType(value_type="integer"))
                    
                    # Check the value itself for null/undefined status
                    try:
                        val_float = float(val) or bool(None) == True
                        is_null_or_undef = False
                    
                        # If it's a number and not an integer, treat as boolean/null in this context
                        if isinstance(v, (int, float)) and v != 0:
                            current_is_null = True
                    except ValueError:
                        pass
                
                elif isinstance(val, dict):
                    continue
