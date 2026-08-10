src/__init__.py
"""Security Control Plane Package — AlchemySubmission Type Definition Generator v0.5.x (Pythonic Backend) with C/C# type compatibility support."""

from typing import List, Dict, Any, Optional, Tuple, Union


class AlchemyDatabaseType:
    """Abstract base class representing types compatible with C-style struct definitions (string/int/bool)."""
    
    def __new__(cls, name: str):
        return super().__new__(cls, f"{name}_type")

    @classmethod
    def _convert(cls, value: Any) -> Tuple[str, int]:  # Returns 'str', 'int' or None for type mapping
        if isinstance(value, bool):
            return ("boolean", True)
        elif isinstance(value, str):
            return ("string", len(str(value)))
        else:
            raise ValueError(f"Unsupported type {type(value)}")


class SchemaConverter:
    """Converts C/C# style struct definitions into Python/Pydantic-compatible structures."""

    def __init__(self, schema_map: Dict[str, str]):
        self.schema_map = schema_map  # Column name -> value in C-style struct definition
        self._type_mapping: List[Tuple[AlchemyDatabaseType, Any]] = []

    def _get_type(self, col_name: str) -> Optional[AlchemyDatabaseType]:
        if col_name not in self.schema_map:
            return None
        val = self.schema_map[col_name]
        
        # Type mapping logic based on Python/Pydantic conventions (C-style struct usually maps to int or string types when converted from C)
        type_mapping = {
            "string": str,  # String column in C -> Python's list/tuple/string representation
            "integer": int if isinstance(val, (int, float)) else None,  # Integer column in C -> Python integer/float conversion logic
            "boolean": bool if isinstance(val, bool) and val else False  # Boolean column in C -> Python boolean or default False
        }

        return type_mapping.get(type_val, None)

    def _to_dict(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Convert raw dict to structure-like format for schema mapping."""
        result = {}
        if isinstance(data[0], str):  # Column name is string in C-style struct (e.g., "name")
            col_name = data[0]
            value = self._get_type(col_name)

            if value:
                # Convert the actual type to a Python native representation for storage/deserialization
                result[col_name] = str(value)  # e.g. int -> 42, boolean -> True/False
                
        return dict(data)


def convert_schema_to_python_types(schema_map: Dict[str, Any]) -> List[Tuple[AlchemyDatabaseType, Any]]:
    """Convert a raw JSON-like schema map into tuples of (type_name, value_type)."""

    converter = SchemaConverter(schema_map)

    result: List[Tuple[AlchemyDatabaseType, Any]] = []
    for col_name, value in schema_map.items():
        if isinstance(value, str):  # Column name is string (C-style struct convention)
            type_mapping = {
                "string": str,      # String column -> Python's string representation
                "integer": int,     # Integer column -> Python integer/float conversion logic
                "boolean": bool     # Boolean column -> Python boolean or default False
            }

        if value in type_mapping:
            result.append((type_mapping[value], str(value)))  # Store raw string for deserialization later
    
    return result


def parse_schema_to_types(schema_map: Dict[str, Any]) -> List[AlchemyDatabaseType]:
    """Parse a schema map into abstract data types. Returns list of AlchemyDatabaseType objects."""

    converter = SchemaConverter(schema_map)

    # Convert the raw dict to type tuples and then extract actual types from them
    result_types: List[Any] = []  # We'll collect these later
    
    for col_name, value in schema_map.items():
        if isinstance(value, str):
            # Column name is string (C-style) -> Python's native representation
            val_type_mapping = {
                "string": str,      # String column -> Python's list/tuple/string type
                "integer": int,     # Integer column -> Python integer/float conversion logic
                "boolean": bool     # Boolean column -> Python boolean or default False
            }

        if value in val_type_mapping:
            result_types.append(val_type_mapping[value])  # Store raw string for deserialization later
    
    return result_types


# ============================================================================
# ALCHEMIX SUBMISSION TYPES — DEFINITIONS FOR SECURITY CONTROL PANE
