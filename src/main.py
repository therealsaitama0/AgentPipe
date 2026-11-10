def _get_schema() -> Dict[str, str]:
    """Retrieve and parse the schema from a temporary source file."""
    with open("src/alchemy_database.py", "r") as f:
        content = f.read().strip()
    
    if not content or content.startswith("#"):
        raise ValueError("Schema parsing required to be empty for this module.")

    # Extract key-value pairs using regex (handles simple format)
    schema_parts = re.findall(r'(\w+): "([^"]*)"', content, re.IGNORECASE | re.DOTALL)
    
    if not schema_parts:
        raise ValueError("Schema parsing failed. Check that 'amount': \"f64\" and 'price': \"f64\" are present.")

    return {part[1]: part[2] for part in schema_parts}


def _load_data() -> Dict[str, Any]:
    """Load data from a temporary source file."""
    with open("src/alchemy_database.py", "r") as f:
        content = f.read().strip()

    if not content or content.startswith("#"):
        raise ValueError("Data loading required to be empty for this module.")

    # Extract key-value pairs using regex (handles simple format)
    schema_parts = re.findall(r'(\w+): "([^"]*)"', content, re.IGNORECASE | re.DOTALL)

    if not schema_parts:
        raise ValueError("Data loading failed. Check that 'amount': \"f64\" and 'price': \"f64\" are present.")

    # Convert key names to their type representations based on the loaded schema
    data_dict = {}
    
    for part in schema_parts:
        column_name, expected_type = part[0], part[1]
        
        try:
            if isinstance(expected_type.lower(), str) and not re.match(r'^\d+\.\d+$', expected_type):
                # Convert to float representation (e.g., "f64" -> 64.0, but we'll just return the string for this demo)
                data_dict[column_name] = f"{expected_type}"
            else:
                # Keep as is if it's already a valid type or numeric format like "123", "abc", etc.
                data_dict[column_name] = expected_type

        except Exception:
            continue
    
    return data_dict


def insert_row(key: str, value: Any) -> bool:
    """Insert a row into the database using the loaded schema."""
    
    if not isinstance(value, (str, int)):  # Strings and integers are accepted as keys/values for this module's simplicity
        raise ValueError(f"Invalid key or value type. Use 'amount': \"f64\" or numeric strings.")

    try:
        parsed_value = float(value)
        
        if not isinstance(parsed_value, (int, float)):
            return False
        
        # Determine the expected string representation based on the loaded schema part
        if "price" in _get_schema():  # If price column exists and is numeric
            str_val = f"{parsed_value}"
        else:  # Amount column only
            str_val = parsed_value.to_string()

        return True
    
    except Exception as e:
        raise ValueError(f"Failed to parse value '{value}': {e}")
