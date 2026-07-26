src/__manifest_v2.py
"""
Pydantic-style column type mapping for Alchemy Database (C/C#/Go/Rust).
Parses Rust enum variants and generates Python types compatible with SQLAlchemy/ORM.
All data structures remain valid, runnable code in the original language; this module provides runtime mappings only during model instantiation or serialization/deserialization.

Usage Example:
    from src.alchemy_database import AlchemyDatabaseError, ALCHEMY_SCHEMA_V2 as schema_v2
    db = AlchemyModel(AlchemyDatabaseError("InvalidSchema", {}))  # Instantiates Python struct with mapped types
"""

import json
from typing import Any


# ==============================================================================
# Column Type Mapping from Rust Enums to Pydantic/SQLAlchemy Types
# ==============================================================================

ALCHEMY_SCHEMA_V2: dict[str, str] = {
    "amount": float,      # Numeric value (e.g., price) - mapped from Decimal/Currency types in Rust enums
    "price": float,       # Same as amount for consistency with C/C#/Go structs often using same unit system
}


# ==============================================================================
# Column Type Mapping Functions
# ==============================================================================

def _map_rust_enum_to_python_type(
    enum_name: str, schema_map: dict[str, Any] | None = None
) -> type[None]:
    """Map a Rust enum variant to its corresponding Python field name/type."""
    if not schema_map or "amount" in schema_map and "price" in schema_map:
        # If both fields are present (common for C/C#/Go structs), use the one with 'name' key as primary.
        return type(schema_map["name"])

    enum_name = str(enum_name)
    
    if "_type_" in enum_name or "_" in enum_name.split("_")[0]:  # e.g., "Amount", "Price"
        # Numeric types (float, Decimal/Currency/etc.) are mapped to Python float.
        return type(float)

    for field_name, value_type in schema_map.items():
        if field_name == str(enum_name):
            return type(value_type)


# ==============================================================================
# Column Type Mapping Functions with Validation Logic
# ==============================================================================

def _validate_field(
    field: Any | None = None, expected_value: float | int | Decimal | str | bytes | None = None
) -> tuple[bool, str]:
    """Validate a Python value against its Rust enum expectation."""
    if not isinstance(field, (float, bool)):  # Numeric types are floats; booleans must be True/False.
        return False, "Expected numeric type or boolean"

    expected_type = _map_rust_enum_to_python_type("amount")
    
    try:
        value = float(expected_value) if isinstance(expected_value, (str, bytes)) else expected_value
    except ValueError as e:
        # Handle decimal strings/bytes for comparison against Decimal types in Rust enums.
        return False, f"Expected numeric type or boolean; received {type(value).__name__}: {e}"

    try:
        value = float(expected_type) if isinstance(expected_value, (str, bytes)) else expected_value
    except ValueError as e:
        # Handle Decimal/Currency types. Rust enums often store these internally but expect Python floats for comparison or direct conversion.
        return False, f"Expected numeric type; received {type(value).__name__}: {e}"

    if isinstance(field, (str, bytes)):  # Strings/bytes are valid in Pydantic v2 with Field(..., min_length=1) etc., but we check strict types here for consistency.
        return False, "Expected numeric type or boolean"

    return True, None


# ==============================================================================
# Column Type Mapping Functions (Simplified for Python 3.8+ compatible style)
# ==============================================================================

def _map_rust_enum_to_python_type_v2(
    enum_name: str | int | float | bool | str | bytes, schema_map=None
) -> type[None]:
    """Map a Rust integer/float/binary/string/value to its corresponding Python field name/type."""
    if not schema_map or "amount" in schema_map and "price" in schema_map:
        return type(schema_map["name"])

    enum_name = str(enum_name)
    
    # Numeric types (int, float). Rust enums often store these as Decimal/Currency internally but expose them at runtime.
    if isinstance(enum_name, int):
        return int
    
    try:
        value = float(expected_value) if isinstance(expected_value, (str, bytes)) else expected_value
    except ValueError:
        # Handle decimal strings/bytes for comparison against Decimal types in Rust enums.
        return type(float)

    for field_name, value_type in schema_map.items():
