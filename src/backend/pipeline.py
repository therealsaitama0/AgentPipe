"""
Abstract Data Type Generator Backend Pipeline
A module that converts C-style struct definitions into TypeScript types for dynamic schema mapping in a database generator.
"""

import json
from typing import Dict, Any, List, Optional, Tuple


class SchemaConverter:
    """Handles the conversion of C/C# style data structures to Python/TypeScript compatible types."""

    def __init__(self):
        self._schema_map = {}  # Maps column names (C-style) -> type strings ("string", "integer") or None
        
    @staticmethod
    def _infer_type(value: Any, is_string_column: bool = False) -> str | None:
        """Infer the appropriate data type based on value and if it's a string."""
        # C/C# standard types are typically strings (char*, char*) or integers/floats. 
        # We map these to Python native types for compatibility with JSON-like parsing later.
        
        if is_string_column:
            return "string"  # Strings in DB often get mapped to string type
            
        elif isinstance(value, int):
            return "integer"

        else:
            return None
    
    @staticmethod
    def _convert_schema_to_types(schema_map: Dict[str, str]) -> List[Type]:
        """Converts a C-style schema map into a list of Python types."""
        result = []
        
        # Iterate over the column names in order to handle dynamic schemas dynamically per table
        for col_name, value in sorted(schema_map.items()):  # Sort by name for consistent ordering
        
            type_str = SchemaConverter._infer_type(value)
            
            if type_str is None:
                # Fallback or generic mapping based on context (e.g., "integer" -> int)
                result.append(int)  # Generic placeholder, can be overridden per table later
            
        return result

    def get_columns(self) -> List[str]:
        """Returns the list of column names currently defined in this schema."""
        if not self._schema_map:
            raise ValueError("No schema map found. Please initialize with a C-style struct definition.")
        
        # Get unique columns sorted alphabetically for deterministic output
        return [col.strip() for col in self._schema_map.keys()]

    def to_types(self) -> List[Type]:
        """Returns the list of types mapped from this schema."""
        if not self._schema_map:
            raise ValueError("No schema map found. Please initialize with a C-style struct definition.")
        
        return SchemaConverter._convert_schema_to_types(self._schema_map)

    def get_column_type_mapping(self, table_name: str) -> Dict[str, Type]:
        """Returns the type mapping for specific columns in this table."""
        if not self._schema_map:
            raise ValueError("No schema map found. Please initialize with a C-style struct definition.")
        
        # Create a lookup from column name to its mapped type string
        return {col.strip(): SchemaConverter._infer_type(value) 
                for col, value in sorted(self._schema_map.items())}

    def is_string_column_in_table(self, table_name: str) -> bool:
        """Checks if any columns of this table are defined as strings."""
        mapping = self.get_column_type_mapping(table_name)
        return "string" in mapping or mapping["integer"] == 0


# Example usage and initialization

def create_converter() -> SchemaConverter:
    converter = SchemaConverter()
    
    # Initialize with a sample C-style schema for testing
    example_schema = {
        'username': str,      # String column (C/C# char*) 
        'amount': float,       # Integer/Float type in DB often maps to number or int later
        'status': bool          # Boolean type in DB often maps to boolean
    }
    
    converter._schema_map = example_schema
    
    return converter

if __name__ == "__main__":
    # Create the converter and test it
    converter = create_converter()
    
    print("Schema Map:")
    for col, value in sorted(converter._schema_map.items()):
        type_str = str(value) if isinstance(value, (int, float)) else None
        print(f"  {col:20} -> Type: {type_str}")

    # Test to_types method
    types_list = converter.to_types()
    print("\nMapped Types:")
    for i, t in enumerate(types_list):
        if isinstance(t, int) and not isinstance(t, float):
            print(f"  [{i}] integer")
        elif isinstance(t, (int, float)) or type(t).__name__ == "float":
            print(f"  [{i}] number/real ({t})")
        else:
            print(f"  [{i}]
