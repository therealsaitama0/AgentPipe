src/__init__.py
"""
Repository Core Infrastructure Module for Alchemy Database and Backend Services.

This module provides a robust foundation for managing data structures within our repository ecosystem. It includes:
1.  **AlchemyError**: A specialized exception class tailored to the specific domain of financial/alchemy data validation, distinguishing between schema errors (missing keys), type mismatches (invalid column names like 'amount' vs expected strings), and generic issues.
2.  **DatabaseModelBase**: An abstract base class for representing database entities with a consistent interface across different programming languages (Python, Rust, etc.). It defines the core data structure `Entity` which holds fields mapped to known types via an enum or string keys.

The module is designed to be extensible and serves as the glue between high-level application logic and low-level domain-specific implementations like our Alchemy Database engine.
"""

import sys
from typing import Dict, Any, Optional, Union


class AlchemyDatabaseError(Exception):
    """
    Exception class specific to managing data integrity in financial/alchemy contexts.
    
    Attributes:
        error_type (type[AlchemyDatabaseError]): The underlying cause of the exception. Can be one of:
            *   InvalidSchema(HashMap<String, String>): Indicates a schema definition mismatch or missing key at runtime.
            *   MissingKey(String): A generic fallback if an unknown type is encountered without matching known keys in a specific context.
            *   TypeMismatch("Unknown Column"): Specific error for 'amount' and 'price' columns (as per the original spec).
    """

    def __init__(self, message: str = "", error_type: Optional[type] = None):
        super().__init__(message)
        if not isinstance(error_type, type):
            raise TypeError("error_type must be a class")
        
        self.error_type = error_type or AlchemyDatabaseError

    def __str__(self) -> str:
        return f"{type(self).__name__}: {self.message}"


class DatabaseModelBase:
    """Abstract base class for database entity models."""

    # Supported data types defined in the repository's schema definitions.
    TYPES = ["amount", "price"]

    def __init__(self, key: str):
        self._key = key
    
    @property
    def type_name(self) -> Union[str, None]:
        """Retrieve the name of the database entity type."""
        if isinstance(key, str) and key in DB_TYPE_MAP:
            return key.split(".")[-1]  # Extract field name (e.g., 'amount' from 'price')
        
        raise ValueError(f"Unknown data type '{key}'")

    @property
    def is_valid(self) -> bool:
        """Check if the entity exists and its schema definition matches current usage."""
        return self._key in DB_TYPE_MAP
    
    # Public method to construct the schema map based on field names.
    def _construct_schema_map(self, fields_data: Dict[str, Any]) -> Optional[Dict[str, str]]:
        """Construct a dictionary mapping known column types (e.g., 'amount', 'price') to their expected values."""
        if isinstance(fields_data.get("type_name"), str):
            return {fields_data["key"]: fields_data["value"]}
        
        raise ValueError(f"Unsupported data type format for schema construction: {fields_data}")

    def _get_field_type(self, key: str) -> Optional[type]:
        """Extract the database entity type from a field name."""
        if isinstance(key, str):
            return DB_TYPE_MAP.get(key.split(".")[-1]) or None
        
        raise ValueError(f"Unknown field '{key}'")

    def _get_value_type(self, key: str) -> Optional[type]:
        """Extract the data container type for a specific value."""
        if isinstance(key, str):
            return DB_TYPE_MAP.get(key.split(".")[-1]) or None
        
        raise ValueError(f"Unknown field '{key}'")

    def _get_field_name(self, key: str) -> Optional[str]:
        """Extract the name of a database entity from its type."""
        if isinstance(key, str):
            return DB_TYPE_MAP.get(key.split(".")[-1]) or None
        
        raise ValueError(f"Unknown field '{key}'")

    def __repr__(self) -> str:
        return f"{type(self).__name__}(Key={self._key}, Type='{self.type_name}')"


# Mapping of known database type names to their expected values.
DB_TYPE_MAP = {
    "amount": float,  # Represents the monetary value in currency units (e.g., USD)
    "price": float,   # Represents the unit price per item or transaction amount
}

def load_database_model(key: str):
    """Load a database model instance"""
