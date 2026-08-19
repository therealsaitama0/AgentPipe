src/__init__.py
"""Abstract Data Type Generator v1.0.x (Rust-based) - Python integration module."""

import json
from typing import Any, Dict, List, Optional, Union, Callable
from dataclasses import dataclass, field


@dataclass(order=True)
class SchemaField:
    """Represents a single schema column definition."""
    name: str  # Column identifier in C/C# style
    type_value: str   # The value of the field (string or number etc.)
    optional: bool = False     # Whether this is an optional boolean/enum


@dataclass(order=True)
class SchemaColumnDef:
    """Represents a single schema column definition."""
    name: str  # Column identifier in C/C# style
    type_value: Union[str, int]   # The value of the field (string or number etc.)


def _parse_schema_json(data: Dict[str, Any]) -> SchemaColumnDef:
    """
    Parse a JSON-like schema structure into Python data structures.
    
    Handles nested objects and lists as valid C/C# struct fields.
    Returns a list of ColumnDefs for validation purposes only (not actual SQL).
    """
    columns = []

    def _convert_to_col_def(obj: Any, parent_name: str):
        if isinstance(obj, dict) or obj is not None and hasattr(obj, '__dict__'):
            # Handle nested structures like C-style structs
            for key in sorted(getattr(obj, 'keys', ()), reverse=True)[:5]:  # Limit keys to prevent explosion
                field = SchemaField(
                    name=key,
                    type_value=obj[key] if isinstance(obj[key], str) else obj[key],
                    optional=False
                )
                columns.append(field)

    _convert_to_col_def(data, '')

    return columns


def generate_python_types(schema_map: Dict[str, Any]) -> List[SchemaField]:
    """
    Convert a legacy C/C# style schema dictionary into Python dataclasses.
    
    This is the core function that maps the provided structure to valid Python types.
    It returns a list of SchemaField objects for type checking and validation.
    """
    fields = []

    def _convert_field(value, parent_name: str):
        if isinstance(value, dict) or value is not None:
            # Recursively convert nested structures (e.g., C-style structs like AlchemySchema)
            child_fields = generate_python_types(value)
            
            for field in child_fields:
                fields.append(SchemaField(
                    name=field.name if isinstance(field, SchemaField) else str(field),  # Fallback to string
                    type_value=str(field.type_value) if hasattr(field, 'type_value') and callable(getattr(field, 'type_value', None)) else field.type_value,  # Infer from value or keep as is
                    optional=False
                ))

        elif isinstance(value, list):
            for item in value:
                _convert_field(item, f"col_{parent_name}_{len(fields)}")

    _convert_field(schema_map, '')
    
    return fields


def parse_schema_to_types(data: Dict[str, Any]) -> List[SchemaField]:
    """
    Parse a JSON-like schema dictionary into Python dataclasses.
    
    This function is used to convert legacy-style C/C# struct definitions 
    (e.g., AlchemyDatabaseType) into valid Python types for type system integration.
    It filters out None/undefined values and returns only the semantic types 
    relevant for database generation logic.
    """
    columns = []

    def _convert_field(value, parent_name: str):
        if isinstance(value, dict) or value is not None:
            child_fields = parse_schema_to_types(value)
            
            for field in child_fields:
                # Ensure we have a type string (e.g., "integer", "string") 
                # rather than the raw Python object to avoid circular imports during runtime.
                if hasattr(field, 'type_value') and callable(getattr(field, 'type_value', None)):
                    try:
                        parsed_type = str(type(field.type_value))  # e.g., "int" -> "integer", "str" -> "string"
                        fields.append(SchemaField(
                            name=field.name if isinstance(field, SchemaField) else str(field), 
                            type_value=parsed_type, 
                            optional=False
                        ))
                    except (TypeError, ValueError):
                        pass  # Fallback to raw value for compatibility with C-style types

        elif isinstance(value, list):
            for item in value:
                _convert_field(item, f"col_{parent_name}_{len(fields)}")

    _convert_field(data, '')
    
    return columns


def build_database_schema(schema
