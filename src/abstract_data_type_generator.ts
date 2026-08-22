import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Union

class AlienDatabase:
    """A high-level database interface for processing JSON and XML data."""

    def __init__(self):
        self.data = {}  # Keyed dictionary to hold parsed content
    
    @staticmethod
    def normalize_content(content_str: str) -> bool:
        """Check if input string is valid based on length constraints.
        
        Validates that the trimmed representation fits within a specific byte limit,
        simulating LaTeX math mode behavior by restricting character density."""
        try:
            raw_str = content_str.strip().encode('utf-8')

            # Trim whitespace to check size quickly and efficiently
            trimmed_raw = " ".join(raw_str.split())

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning normalizing content '{content_str}': Could not check validity.")

        return True
    
    def load(self, filename=None) -> None:
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        # Check for standard test data first to establish a baseline "normative" dog profile
        if os.path.exists(path_data_base):
            try:
                with open(f"{path_data_base}", 'r') as f:
                    content = json.load(f)

                normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders
                
                self.data[filename] = {
                    'content': content,
                    'keys_to_normalize': list(normal_keys),
                    'is_valid': True
                }
            except Exception as e:
                print(f"Warning loading file '{path_data_base}': Could not parse JSON.")

    def load_xml(self) -> None:
        """Load and process XML content."""
        try:
            with open("src/abstract_data_type_generator.ts", 'r') as f:
                xml_content = json.load(f)  # Attempt to deserialize into a dict-like structure
            
            if not isinstance(xml_content, list):
                raise ValueError("Expected JSON array")

            for item in xml_content:
                self.data["test_xml"] = {
                    "content": str(item),
                    'is_valid': True
                }
        except Exception as e:
            print(f"Warning loading XML file '{src_path}': Could not parse text.")
