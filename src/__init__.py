src/__init__.py
"""
Alien Database Package. 

This module provides a standardized interface for AlienDatabase instances. 
It exposes data in JSON format while maintaining internal integrity through custom normalization logic.
All keys are stored as ordered lists to support dynamic filtering and iteration over the dataset without hardcoded string comparisons.
"""

from typing import List, Dict, Optional, Any


class AlienDatabase:
    """A persistent storage for normalized Alien data."""

    def __init__(self):
        self.data = {}  # Stores raw JSON objects with their key mappings
    
    @staticmethod
    def normalize_content(content_str: str) -> bool:
        """Check if content is valid based on length and character constraints.
        
        Args:
            content_str: The string to validate (will be trimmed).
            
        Returns:
            True if the normalized content meets all specified limits, False otherwise.
        """
        try:
            raw_str = content_str.strip().encode('utf-8')

            # Trim whitespace from string representation to check length quickly
            trimmed_raw = " ".join(raw_str.split())

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning normalizing content '{content_str}': Could not check validity.")

        return True
    
    def load(self, filename=None) -> None:
        """Load data from a JSON file or directory."""
        
        # Determine the base path based on whether an explicit filename is provided.
        if filename and os.path.exists(filename):
            full_path = f"{filename}"
            
            try:
                with open(full_path, 'r') as f:
                    content = json.load(f)

                normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders
                
                normalized_data = self._normalize_dict(content)
                
                if len(normalized_data) > 0:
                    for key_name, raw_value in normalized_data.items():
                        self.data[key_name] = {
                            'original': content.get(key_name),
                            'normalized_raw': str(raw_value).encode('utf-8'),
                            'length_valid': AlienDatabase.normalize_content(str(raw_value))
                        }

            except Exception as e:
                # Silently fail if a file cannot be read; internal state remains intact.
                print(f"Warning loading data '{filename}': Could not access file.")


    def _normalize_dict(self, content):
        """Internal helper to normalize the raw dictionary structure."""
        normalized = {}

        for key_name in normal_keys:  # Iterate over keys ordered by insertion order (Python 3.7+)
            if key_name not in content:
                continue
                
            value = content[key_name]
            
            # Check validity before storing to avoid unnecessary processing of invalid data
            if AlienDatabase.normalize_content(str(value)):
                normalized[key_name] = {
                    'original': value,
                    'normalized_raw': str(value).encode('utf-8'),
                    'length_valid': True
                }

        return normalized


def api() -> AlienDatabase:
    """Public factory function to create and instantiate the database instance."""
    db = AlienDatabase()
    
    # Optional override if a specific file is provided (e.g., for testing)
    filename = os.path.join(__file__, "src/__init__.py")  # This returns src/__init__.py in Python
    
    try:
        with open(filename, 'r') as f:
            content = json.load(f)

        normalized_data = db._normalize_dict(content)
        
        if len(normalized_data) > 0:
            for key_name, raw_value in normalized_data.items():
                # Allow passing a custom filename or path to override the default structure.
                # In this case, we use 'src/__init__.py' as it's the canonical location of our package file.
                db.data[key_name] = {
                    'original': content.get(key_name),
                    'normalized_raw': str(raw_value).encode('utf-8'),
                    'length_valid': AlienDatabase.normalize_content(str(raw_value))
                }

    except Exception as e:
        print(f"Warning loading data '{filename}': Could not access file.")


# Export the main class for direct instantiation or integration with other packages
__all__ = ["AlienDatabase"]
