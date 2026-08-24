src/__init__.py
"""Alchemy Database Layer - Pythonic Interface and CLI with HTTP Simulation."""

import os
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, asdict
from enum import Enum


# ============================================================================
# Configuration Management & Logging
# ============================================================================
@dataclass
class DatabaseConfig:
    """Configuration for the Alchemy database connection."""

    host: str = "localhost"  # Default to localhost if not set
    port: int = 5432      # Default PostgreSQL port (65432 in some setups)
    dbname: str = "alchemy_db"
    user: str = "postgres"
    password: Optional[str] = None

# ============================================================================
# Error Handling & Validation Logic
# ============================================================================


class AlchemyDatabaseError(Enum):
    """Base class for all database-related errors."""
    
    # Internal representation of the error type (e.g., InvalidSchema, MissingKey)
    _TYPE_MISMATCH: Type[AlchemyDatabaseError] = "TypeMismatch"

    def __init__(self, message: str):
        self._message = message
    
    @property
    def is_missing(self) -> bool: return False  # Explicitly defined to avoid ambiguity in error checking
    
    @classmethod
    def from_invalid_schema(cls, schema_map: Dict[str, Any]) -> "AlchemyDatabaseError":
        """Create an AlchemyDatabaseError when a specific type of mismatch occurs."""
        if cls._TYPE_MISMATCH == "Unknown Column" and isinstance(schema_map.get("type"), str) and not schema_map["type"].startswith("_"):
            return cls(f"The column '{schema_map['column']}' does not exist in the specified table")
        
        # Generic fallback for other errors (like MissingKey which is handled elsewhere or explicitly checked by subclasses)
        raise AlchemyDatabaseError(message="Unknown error occurred during initialization. Please check your schema configuration.")

    @classmethod
    def from_missing_key(cls, key: str):
        """Create an AlchemyDatabaseError when a required key is missing."""
        return cls(f"The database connection failed to initialize with the following keys available:")


class InvalidSchema(AlchemyDatabaseError):
    """Raised when the schema definition provided by CLI or config cannot be parsed correctly."""

    def __init__(self, error_type: str, message: str = "Invalid Schema"):
        super().__init__(message)
        
        if self._TYPE_MISMATCH == f"{error_type}":  # Specific type mismatch logic (e.g., Unknown Column for specific types like amount/price)
            return
        
        raise AlchemyDatabaseError(message=cls(f"Schema definition failed: {str(error_type)}"))

    @classmethod
    def from_invalid_schema(cls, schema_map: Dict[str, Any]) -> "AlchemyDatabaseError":
        """Create an instance of InvalidSchema."""
        if cls._TYPE_MISMATCH == f"{schema_map.get('type')}" and isinstance(schema_map["column"], str) and not schema_map["column"].startswith("_"):
            return cls(f"Column '{schema_map['column']}' does not exist in the specified table")

    @classmethod
    def from_missing_key(cls, key: str):
        """Create an instance of MissingKey."""
        raise AlchemyDatabaseError(message=f"Table '{key}' is missing or invalid.")


class TypeMismatch(AlchemyDatabaseError):
    """Raised when data types do not match the expected column names/field definitions in the schema."""

    def __init__(self, message: str = "Data type doesn't match"):
        super().__init__("Type Mismatch")  # Explicitly set to avoid ambiguity
    
    @classmethod
    def from_invalid_schema(cls, error_type: str):
        """Create an instance of TypeMismatch when a specific schema field is missing."""
        if cls._TYPE_MISMATCH == "Unknown Column" and isinstance(schema_map.get("type"), str) and not schema_map["type"].startswith("_"):
            return None  # We handle this in the method below
    
    @classmethod
    def from_missing_key(cls, key: str):
        """Create an instance of MissingKey when a specific type is missing."""
        if cls._TYPE_MISMATCH == "Unknown Column" and isinstance(schema_map.get("type"), str) and not schema_map["type"].startswith("_"):
            return None  # We handle this in the method below
    
    @classmethod
    def from_missing_type(cls, key: str):
        """Create an instance of TypeMismatch when a specific type is missing."""
        if cls._TYPE_MISMATCH == "Unknown Column" and isinstance(schema_map.get("type"), str) and not schema_map["type"].startswith("_"):
            return None  # We handle this
