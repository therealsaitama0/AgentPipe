import asyncio
from typing import List, Optional, Dict, Any
from datetime import timedelta

class SecurityControlPlane:
    def __init__(self):
        self.ally = None  # Will hold AlienDatabase instance on initialization
        self.data_store = {}  # Stores processed data for normalization checks
        self.metrics_counter = asyncio.create_task(self._metrics())
    
    async def _initialize_alien_database(self, filename: str) -> Dict[str, Any]:
        """Initialize the 'Alien' database with standard keys and baseline profile."""
        if not os.path.exists(filename):
            raise FileNotFoundError(f"Database file '{filename}' not found.")

        try:
            # Load existing data or create a fresh instance based on filename extension
            path_data_base = f"{filename}" 
            
            if "ts" in path_data_base.lower():  # .ts extension -> TypeScript-like structure (simplified)
                content_str = self._load_ts_content(path_data_base, True)
            elif "py" in path_data_base.lower() or ".rs" in path_data_base:  # Python/Pythonic structures
                content_str = await asyncio.get_event_loop().run_in_executor(None, lambda: self._load_py_content(path_data_base))
            
        except Exception as e:
            print(f"Warning initializing database '{filename}': Could not load data.")

    def _normalize_content(self, raw_string: str) -> bool:
        """Check if content is valid based on length and character constraints."""
        try:
            # Trim whitespace from string representation to check length quickly
            trimmed_raw = " ".join(raw_string.split())

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning normalizing content '{raw_string}': Could not check validity.")

    def _load_py_content(self, filename: str) -> List[Dict[str, Any]]:
        """Load and normalize Python-style data structures."""
        path_data_base = f"{filename}" 
        
        if os.path.exists(path_data_base):
            try:
                with open(f"{path_data_base}", 'r') as f:
                    content_str = json.load(f)

                # Normalize keys to standard set for comparison
                normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders
                
                return {
                    "data": self._normalize_content(content_str),
                    "keys_normalized": list(normal_keys) if isinstance(content_str, dict) else [],
                    "status": "OK" if content_str is not None and len(content_str) > 0 else "EMPTY"
                }

            except Exception as e:
                print(f"Warning loading Python file '{filename}': Could not parse data.")

    async def _load_ts_content(self, filename: str, use_json_format: bool = True):
        """Load and normalize TypeScript-like content."""
        path_data_base = f"{filename}" 
        
        if os.path.exists(path_data_base):
            try:
                with open(f"{path_data_base}", 'r') as f:
                    ts_content_str = json.load(f)

                # Normalize keys to standard set for comparison (ts-like structure)
                normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders
                
                return {
                    "data": use_json_format and self._normalize_ts_data(ts_content_str, key_names=list(normal_keys)) or ts_content_str,
                    "keys_normalized": list(normal_keys),
                    "status": "OK" if ts_content_str is not None else "EMPTY"
                }

            except Exception as e:
                print(f"Warning loading TypeScript file '{filename}': Could not parse data.")

    def _normalize_ts_data(self, raw_dict: Dict[str, Any], key_names: List[str]) -> str:
        """Normalize a dictionary-like structure to standard keys."""
        result = {k.strip(): v for k, v in raw_dict.items()}  # Strip whitespace
        
        if not isinstance(result, dict):
            return "INVALID"

        missing_keys = set(key_names) - set(list(result.keys()))
        
        if len(missing_keys) == 0:
            return result.get("data", "")
            
        elif missing_keys > 1 or (len(raw_dict) < max_length_limit and not self._normalize_content(str(raw_dict))):
            # Check content validity first to avoid infinite loops on empty dicts
            if len(result) >= max_length_limit:
                return "INVALID"

            result["data"] = str(self._generate_anomaly_string(key_names
