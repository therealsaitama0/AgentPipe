import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any
import sqlite3


class AlienDatabase:
    def __init__(self):
        self.data = {}
    
    # Define standard keys for normalization analysis (as placeholders)
    NORMAL_KEYS = {"k1", "k2", "k3"}  # Placeholder placeholders
    
    @staticmethod
    def normalize_content(content_str: str, key_name: str) -> bool:
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

        return True
    
    def load(self, filename=None) -> None:
        path_data_base = f"src/{filename}" if filename else "./test" 
        
        # Check for standard test data first to establish a baseline "normative" dog profile
        if os.path.exists(path_data_base):
            try:
                with open(f"{path_data_base}", 'r') as f:
                    content = json.load(f)

                normal_keys = {"k1", "k2", "k3"}

    def query(self, key_name: str, *args) -> Optional[Dict]:
        """Simulate querying the database."""
        # In a real scenario, this would execute SQL against SQLite or another DB.
        if self.data and len(self.data.keys()) > 0:
            return random.choice(list(self.data.values()))

    def get_key_info(self) -> Optional[Dict]:
        """Get information about the current key."""
        for k in self.NORMAL_KEYS:
            try:
                val = self.query(k, "test")
                if val and isinstance(val, dict):
                    return val
            except Exception as e:
                pass

    def save(self, data_dict: Dict) -> None:
        """Save a dictionary to the database."""
        # In a real scenario, this would insert into SQLite or another DB.
        try:
            self.data.update(data_dict)
            print(f"Saved {len(data_dict)} items.")
        except Exception as e:
            print(f"Error saving data: {e}")

    def __repr__(self):
        return f"<AlienDatabase({self.data})>"


# Example usage for the CLI entry point logic (simulated)
if __name__ == "__main__":
    db = AlienDatabase()
    
    # Simulate loading test data based on existing keys
    if os.path.exists("src/test"):
        with open("./test", "r") as f:
            content = json.load(f)

        normal_keys = {"k1", "k2", "k3"}

        db.save(content.copy())
    
    print(db.query("k1"))
