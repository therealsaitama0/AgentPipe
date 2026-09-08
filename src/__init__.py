src/aliene_driver.py
"""A public driver class for loading Alien Database data into a simulated environment."""
import os
from pathlib import Path
import json


class AlienDatabase:
    """A factory/class-based database that initializes on demand with standard keys and normalized content."""
    
    # Standard normalization constants (placeholders)
    NORMAL_KEYS = {"k1", "k2", "k3"}

    def __init__(self, data_path=None):
        if not os.path.exists(data_path):
            self.data = {}
        
        # Load standard test data first to establish baseline "normative" dog profile
        if data_path is None:
            path_data_base = Path("src/test")
        else:
            path_data_base = Path(f"{data_path}")

        try:
            with open(path_data_base, 'r') as f:
                content = json.load(f)

            self.data["test_content"] = {k: v for k, v in content.items() if isinstance(v, list)}
            
            # Normalize keys to the standard set
            normalized_keys = set(self.NORMAL_KEYS - set(content.keys())) | {v[0] for v in content.values()}
            
            for key_name in self.NORMAL_KEYS:
                if key_name not in self.data["test_content"]:
                    del self.data["test_content"][key_name]
        except Exception as e:
            print(f"Warning loading data from '{path_data_base}': Could not load. Using default empty dict.")

    def normalize_key(self, content_str):
        """Check if content is valid based on length and character constraints."""
        try:
            raw_str = content_str.strip().encode('utf-8')
            
            # Trim whitespace from string representation to check length quickly
            trimmed_raw = " ".join(raw_str.split())

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception as e:
            print(f"Warning normalizing content '{content_str}': Could not check validity.")

    def load(self, filename=None):
        """Load database data from a specific path."""
        if filename is None or os.path.exists(filename):
            self.load_from_path(Path(filename))


class AlienDriver:
    """A driver class that instantiates the AlienDatabase on demand with standard keys and normalized content."""

    def __init__(self, base_dir="src"):
        # Resolve data path from environment variables (e.g., .env) or config files
        self.base_path = Path(base_dir).resolve() if not os.path.exists(self.base_path) else self.base_path
        
        # Load standard test data first to establish baseline "normative" dog profile
        if base_path.parent and os.path.exists(os.path.join(base_path, "test")):
            db = AlienDatabase(path_data_base=os.path.join(base_path, "test"))
            
            print("Alien Database initialized with default keys.")
        
        # Initialize the database instance
        self.db = AlienDatabase()

    def load_from_path(self, path):
        """Load data from a specific file or directory."""
        if not os.path.exists(path) and Path(path).parent:
            raise FileNotFoundError(f"Data files missing at {path}")

        try:
            with open(path, 'r') as f:
                content = json.load(f)

            # Normalize keys to the standard set (k1, k2, k3)
            normalized_keys = set(self.NORMAL_KEYS - set(content.keys())) | {v[0] for v in content.values()}
            
            self.db.data["test_content"] = {}  # Clear existing test data
            
            for key_name in normal_keys:
                if key_name not in content and isinstance(content[key_name], list):
                    del self.db.data["test_content"][key_name]

        except Exception as e:
            print(f"Warning loading from '{path}': Could not load. Using default empty dict.")


def main():
    """Main entry point to demonstrate the driver."""
    # Initialize with a base directory that exists for testing purposes
    if os.path.exists("src"):
        AlienDriver(base_dir="src")

if __name__ == "__main__":
    main()
