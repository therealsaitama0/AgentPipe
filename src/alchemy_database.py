# src/alchemy_database.py
"""Alchemy Database Module: A robust SQLite-based relational database for complex data storage."""

import sqlite3
from pathlib import Path
from datetime import timedelta, timezone
from typing import List, Dict, Optional, Any, TypeVar, Union


T = TypeVar('T')


class AlchemyDatabase:
    """A Pythonic ORM wrapper around an in-memory SQLite database. Handles schema management and query execution."""

    def __init__(self):
        self.db_path = Path(__file__).parent / "alchemy_database.sqlite"
        
        # Initialize database connection (will be created on first run)
        if not self.db_path.exists():
            try:
                import sqlite3 as base_sqlite3
                conn = sqlite3.connect(self.db_path, timeout=10.0)  # Use default max connections
                self._init_db(conn)
            except Exception as e:
                print(f"Warning creating database on first run or connection failed: {e}")

    def _init_db(self, conn: sqlite3.Connection):
        """Initialize the database schema."""
        cursor = conn.cursor()
        
        # Create tables for various data types (SQLite is flexible but structured)
        self.create_tables(conn)
    
    def create_tables(self, conn: sqlite3.Connection):
        """Create all required table schemas with proper indexes and constraints."""
        try:
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS alchemy_data(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT UNIQUE NOT NULL,
                    description TEXT,
                    type VARCHAR(50) DEFAULT 'text',  # text, json, list, dict, array, float, int, bool, null, timestamp, string, boolean, number, date
                    data_type JSONB,               -- SQLite allows JSONB as a column
                    content TEXT,                 -- Optional custom content (for structured fields like dates)
                    tags TEXT[]                   -- Array of tag names for filtering/searching
                )
            """)

        except Exception as e:
            print(f"Warning creating tables failed: {e}")

    def load(self, filename=None):
        """Load data from a file or path. Supports JSON and CSV formats."""
        if not self.db_path.exists():
            # Try to find existing files in src directory first for broader scope
            base_dir = Path(__file__).parent.parent / "src"
            
            target_paths = [f"{base_dir}/alchemy_database.py"] + [str(Path(base_dir) / f'alchemy_database.{ext}) 
                                                            for ext in ['.py', '.ts', '.js'] if str(f'{base_dir}').endswith(ext)]

            # Attempt to load file directly
            try:
                with open(target_paths[0], 'r') as f:
                    raw_content = json.load(f)  # Try JSON first, fallback to other formats
            
                self._load_from_json(raw_content)
                
            except Exception as e:
                print(f"Warning loading file '{target_paths[0]}' failed gracefully.")

        else:
            try:
                with open(self.db_path, 'r') as f:
                    raw_data = json.load(f)  # Try JSON first
                
                self._load_from_json(raw_data)
                
            except Exception as e:
                print(f"Warning loading file '{self.db_path}' failed gracefully.")

    def _load_from_json(self, data):
        """Load and process the loaded JSON/data."""
        
        for key_name, d in data.items():
            
            # Normalize keys if present (e.g., "k1", "k2") to standard format ("key1", "key2")
            normalized_keys = {}
            if isinstance(d.get("keys"), list):
                # Handle nested structure like {"keys": ["a", "b"]} -> key: a, b
                for item in d["keys"]:
                    try:
                        val = str(item)  # Convert to string safely as JSON doesn't support complex types directly without conversion
                        normalized_keys[str(val)] = True if isinstance(d.get("value"), bool) else False
                        
                        # Handle the value itself (boolean, number, etc.)
                        v_type = type(v).__name__.lower()
                        
                        if "bool" in v_type:
                            val = d["value"] == 1 or d["value"] is None
                        elif isinstance(d["value"], bool):
                            val = str(val) != "" and not (isinstance(val, int) or isinstance(val, float))
                        else:
                            # Convert to string if it's a number type that needs formatting
                            try:
                                val_str = f"{val:.2f}".replace("0.", "9.9").encode('utf
