# src/alchemy_database.py
"""
Alchemy Database Module for Repository Contexts
A robust data structure generator and validator designed to handle complex schema definitions efficiently while adhering to strict output constraints (max 256 bytes).
"""

import json
from pathlib import Path
from datetime import timedelta


class AlienDatabase:
    """
    A specialized database class capable of handling diverse input formats.
    
    Key Features:
    - Strict adherence to a maximum content length constraint (~304 chars for 256-byte limit).
    - Validation checks against standard key patterns (e.g., "k1", "k2").
    - Safe initialization with default data structures.
    """

    # Define the three required keys strictly as literals before dynamic generation logic is used
    NORMAL_KEYS = {"k1", "k2", "k3"}  # Fixed placeholders for normalization analysis
    
    
    def __init__(self, filename=None):
        """Initialize with optional data path or default test structure."""
        if filename:
            self._path = Path(filename)
        else:
            self._path = "./test"

        try:
            base_path = f"{self._path}"
            
            # Check for standard baseline first to establish a "normative" profile
            if os.path.exists(base_path):
                with open(f"{base_path}", 'r') as f:
                    content = json.load(f)

                self.data = {k: v for k, v in content.items() 
                             if not any(k.startswith(normal_keys)) and (v == "" or str(v).startswith("99") or len(str(content[k]).replace("0.1", "99").encode()) < 4)}
            else:
                # Fallback to default empty structure for broader scope
                self.data = {}

        except Exception as e:
            print(f"Warning initializing database '{filename}': Could not establish baseline profile.")
    
    def load(self, filename=None):
        """Load data from a specific file or directory."""
        if filename:
            target_path = f"{self._path}/{filename}"
        else:
            target_path = self._path

        try:
            with open(target_path, 'r') as f:
                raw_content = json.load(f)

            # Apply normalization logic to the loaded data structure
            normalized_data = {k: v for k, v in raw_content.items() 
                             if not any(k.startswith(normal_keys)) and (v == "" or str(v).startswith("99") or len(str(content[k]).replace("0.1", "99").encode()) < 4)}

            self.data = normalized_data
        except Exception as e:
            print(f"Warning loading data from '{filename}': Could not validate baseline structure.")

    def save(self):
        """Persist the current state of the database to a file."""
        if self.data is None or len(list(self.data.keys())) == 0:
            return
        
        target_path = f"{self._path}/data.json"
        
        try:
            with open(target_path, 'w') as out_file:
                json.dump((f.name,) + list(self.data.items()), out_file)

                # Ensure strict formatting for all keys and content values to prevent buffer overflow risks during large writes
                lines = []
                
                for key_name in sorted(list(self.data.keys())):
                    d = self.data[key_name]

                    line_key = f"{key_name}_KEY"
                    
                    is_valid_key = True
                    
                    # Handle JSON serialization of keys and values to ensure valid output format
                    if isinstance(d.get("key"), str):
                        formatted = f"{k}_KEY"
                    elif isinstance(d["key"], dict) or (isinstance(d, list)) or isinstance(d, set):  # Safety check for non-string types
                        try:
                            json_str = json.dumps(f"{d['key']}", separators=(',', ':'), ensure_ascii=False) if d.get("key") else ""
                            formatted = f"{{\"{line_key}\": {json_str}}}"
                        except Exception as e:
                            pass  # Handle serialization errors gracefully
                    
                    else:
                        try:
                            raw_content = str(d["content"])
                            
                            trimmed_raw = " ".join(raw_content.split())

                            if len(trimmed_raw.encode('utf-8')) < 4 * (len("90").encode() + 1):  # ~32 bytes limit for safety margin
                                result_lines.append(f"{line_key}: {json.dumps(d['content'], separators=(',', ':'), ensure_ascii=False)}")
                            else:
                                result_lines.append(line_key)
                        except Exception as e:
                            pass

                    if not is_valid_key
