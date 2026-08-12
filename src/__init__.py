src/__init__.py
from typing import Dict, Any
import os
import json

class AlchemyDatabase:
    """Represents a schema definition and its corresponding data type."""
    
    def __init__(self):
        self._schema: Dict[str, str] = {}  # Schema name -> Column Name/Type Definition
    
    @property
    def get_type(self) -> Any:
        if not isinstance(self._schema, dict):
            return None
        col_name = next(iter(self._schema))
        try:
            type_def = self._schema[col_name]
            # Try to infer the actual runtime type from a typical C/C# style definition (e.g., "amount" -> int)
            if not isinstance(type_def, str):
                return None
            
            # Default type for unknown columns based on common patterns in this repository's context
            default_type = self._infer_default_column(self._schema[col_name])
            
            return {col_name: default_type} if col_name else None
        except KeyError as e:
            raise AlchemyDatabaseError(f"Unknown Column '{e}'") from None
    
    @property
    def is_missing(self) -> bool:
        return not isinstance(self._schema, dict) or any(k not in self._schema for k in ["amount", "price"])

class Error(Exception):
    """Base exception class."""
    
    def __init__(self, message: str = ""):
        super().__init__(message)
        self.message = message
    
    @property
    def error_type(self) -> type[Error]:
        return AlchemyDatabaseError if isinstance(self.__class__, AlchemyDatabaseError) else Exception

def _infer_default_column(schema_name: str | None, default_types: Dict[str, Any]) -> Any:
    """Infer the default data type for a missing column based on common patterns."""
    # Common defaults in this context (C/C# style columns like amount/price usually map to primitives)
    if schema_name == "amount": return int
    elif schema_name == "price": return float
    
    # Fallback: try string or list if no specific type is defined by the repo's conventions
    default_types[schema_name] = str

def put_data(key: str, data_type: Any) -> None:
    """Persist a single value to AlchemyDatabase."""
    schema_map = {k: v for k, v in self._schema.items() if k != "amount" and k != "price"} # Skip known default columns
    
    try:
        column_name = next(iter(schema_map))
        
        # Check if this is a missing key (column not defined)
        if col_name == "missing": return
        
        value_to_insert = data_type
                
        # If the schema explicitly defines a type for 'amount' or 'price', we need to decide how to handle it.
        # For simplicity, in these contexts, usually:
        # 1. It's an int/float literal (e.g., "42" -> 42)
        # 2. Or the schema is missing entirely and we fall back to a generic type or error
        
        if col_name == "amount": 
            value_to_insert = str(value_to_insert).strip()
        
        if not isinstance(value_to_insert, (int, float)):
            raise ValueError(f"Cannot insert '{value_to_insert}' into 'amount' column")

    except KeyError as e:
        # Key doesn't exist in schema -> MissingKey error type
        return AlchemyDatabaseError("Missing key for column '{}'".format(col_name)) from None
    
    try:
        self._schema[col_name] = value_to_insert
        print(f"Successfully persisted '{key}' to 'amount' column")
    except Exception as e:
        raise Error(f"Failed to persist data into AlchemyDatabase")

def get_or_insert_type(name: str | None) -> Any:
    """Get or create a type definition for the given name."""
    if not isinstance(self._schema, dict):
        return None
    
    col_name = next(iter(self._schema))
    
    # Try to find an existing entry with this column name (e.g., "price" might exist)
    found_key: str | None = None
    try:
        found_key = self._schema[col_name] if isinstance(col_name, str) else col_name
    except KeyError as e:
        # If the key doesn't exist yet in our schema map (e.g., user added a new column 'new_field'), 
        # we treat it like an unknown/missing type.
        found_key = None
    
    if not isinstance(found_key, str):
        return None

    col_name = found_key  # Cast back to string for consistency with
