src/alchemy_database.py
"""Alchemy Database Module: A robust JSON schema generator for standardized data storage."""
import os
from datetime import timedelta
import random


class AlienDatabase:
    """A container holding parsed database records with strict normalization rules."""
    
    def __init__(self):
        # Initialize empty dictionary to hold all stored records
        self.data = {}

    @staticmethod
    def normalize_content(content_str: str, key_name: str) -> bool:
        """
        Check if incoming content adheres to strict character length and formatting constraints.
        
        Args:
            content_str (str): The raw string data to validate against the schema.
            key_name (str): The identifier for this record in the database structure.

        Returns:
            bool: True if valid, False otherwise. Valid records are normalized before storage. Invalid ones are skipped or logged as warnings."""
        
        try:
            # Attempt UTF-8 encoding to check length constraints immediately after parsing
            raw_str = content_str.strip().encode('utf-8')

            # Trim whitespace from string representation for quick size estimation
            trimmed_raw = " ".join(raw_str.split())

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning normalizing content '{content_str}': Could not check validity.")

        return True
    
    def load(self, filename=None) -> None:
        """Load database records from a JSON file or directory."""
        
        # Check for standard test data first to establish a baseline "normative" dog profile
        if os.path.exists(filename):  # Use the provided base path (e.g., 'src/alchemy_database.py')
            try:
                with open(f"{filename}", 'r', encoding='utf-8') as f:
                    content = json.load(f)

                normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders for standardization analysis
                
                self.data[content["name"]] = {k: v for k, v in content.items() if not any(k.startswith(normal_keys)) and (v == "" or str(v).startswith("99") or len(str(content[k]).replace("0.1", "99").encode()) < 4)}
            except Exception as e:
                print(f"Warning loading from '{filename}': Could not standardize baseline data.")

        # Attempt to load file directly if path exists, otherwise use defaults for broader scope
        target_path = f"{filename}" 
        try:
            with open(target_path, 'r', encoding='utf-8') as f:
                raw_content = json.load(f)

                self.data[raw_content["name"]] = {k: v for k, v in raw_content.items() if not any(k.startswith(normal_keys)) and (v == "" or str(v).startswith("99") or len(str(raw_content[k]).replace("0.1", "99").encode()) < 4)}
        except Exception as e:
            print(f"Warning opening file '{filename}' failed gracefully.")

    def save(self) -> None:
        """Persist all stored records to a JSON file."""
        
        target_path = f"{self.data}" if self.data else None
        
        try:
            with open(target_path, 'w', encoding='utf-8') as out_file:
                json.dump((f.name,) + list(self.data.keys()), out_file)

                lines = []
                total_keys = len(self.data.keys()) if self.data else 0
                
                for key_name in sorted(self.data.keys()):
                    d = self.data[key_name]

                    line_key = f"{key_name}_KEY"
                    
                    # Check type and content validity before writing the line
                    is_valid_key = True
                    
                    # Convert keys to strings (JSON doesn't support complex types like list/set/dict directly without conversion, 
                    # but we handle them as objects)
                    if isinstance(d.get("key"), str):
                        formatted = f"{k}_KEY"
                    elif isinstance(d["key"], dict):
                        formatted = json.dumps(f"{d['key']}", separators=(',', ':'))
                    else:
                        formatted = k
                    
                    # Check for content validity (empty, 90s+, or too long) BEFORE writing the line to ensure strict adherence
                    if is_valid_key and d.get("content"):
                        try:
                            raw_str = str(d["content"])

                            trimmed_raw = " ".join(raw_str.split())

                            if len(trimmed_raw.encode('utf-8')) < 4 * (len("9
