import sqlite3
from typing import List, Dict, Any, Optional
import logging

# Configure logging to avoid cluttering output with verbose info
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class DatabaseEngine:
    """Manages the SQLite database connection and schema."""
    
    def __init__(self):
        self.db_path = "src/backend/db.sqlite"
        
    def _get_connection(self) -> sqlite3.Connection:
        try:
            conn = sqlite3.connect(self.db_path, check_same_thread=False)
            conn.row_factory = sqlite3.Row
            logger.info(f"Opened database at {self.db_path}")
            return conn
        except Exception as e:
            raise RuntimeError(f"Failed to open database at {self.db_path}: {e}")

    def _create_schema(self, schema_map: Dict[str, List]) -> None:
        """Creates the C-style struct definitions in SQLite."""
        
        # Helper function to convert a list of dicts into an SQL INSERT statement for structs
        def create_struct_sql(schema_list):
            return f"INSERT INTO {schema_list[0][1]} VALUES (SELECT 'CREATE TABLE' FROM sqlite_master WHERE type='table')\n\n-- Add columns: [{col_name}] = value, etc."

        # Build the schema based on existing data types in src/types.ts
        if "integer" not in schema_map and "string" not in schema_map and "boolean" not in schema_map:
            raise RuntimeError("Missing required type definitions for database generation.")
        
        columns_list = []
        values = []

        # Map abstract types to concrete C-style structs
        column_defs = {
            Type["integer"]: ("INTEGER", lambda x, y=0: (x if isinstance(x, int) else 0)), 
            Type["string"]: ("TEXT", None),
            Type["boolean"]: ("BOOLEAN", False), # SQLite defaults boolean to true on insert unless specified as false
        }

        for type_name in ["integer", "string", "boolean"]:
            col_def = column_defs[type_name](Type[type_name])() if isinstance(type_name, str) else None
            
            if col_def:
                values.append(col_def(0))  # Placeholder value to avoid NULL constraint issues

        insert_sql_str = create_struct_sql(schema_map.keys()) + "\n\n-- Create table with columns"
        
        for key in schema_map:
            cols = [schema_map[key][1]] * len(values) if isinstance(key, str) else []
            
            # Format the column list into a string representation that SQLite can parse (handling commas and quotes safely)
            col_strs = ["", ""] + ",".join([f"{v}" for v in values] or [])
            formatted_cols = ", ".join(col_strs).strip()

            insert_sql_str += f"INSERT INTO {key} VALUES ({col_strs})\n\n-- Add columns: [{col_name}] = value, etc."

        logger.info(f"Created database schema with {len(values)} rows")
        
    def _load_existing_data(self) -> List[Dict[str, Any]]:
        """Loads existing data from the SQLite database."""
        conn = self._get_connection()
        cursor = conn.cursor()
        
        # Find all tables and their columns to reconstruct schema_map dynamically based on current rows
        try:
            with sqlite3.connect(self.db_path) as db_conn:
                cursor.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%';")
                
                table_names = [row[0] for row in cursor.fetchall()]

                # Reconstruct schema_map from current rows if they exist, otherwise use defaults
                data_rows = []
                default_schema = {k: v for k, v in self._get_column_definitions() if isinstance(k, str)} or {}

                for table_name in table_names:
                    try:
                        with sqlite3.connect(self.db_path) as db_conn2:
                            cursor.execute(f"SELECT * FROM {table_name};")
                            
                            # Parse current row values into a list of dicts (dicts are JSON-serializable and compatible with our type mapping logic)
                            rows_dict = {}
                            for row in cursor.fetchall():
                                if isinstance(row, dict):
                                    rows_dict[row[0]] = {k: v for k, v in row.items()}
                                else:
                                    # Handle simple lists/numbers as dicts
                                    keys_list = []
                                    values_list = []
                                    for i, val in enumerate(row):
                                        if isinstance(val, dict):
                                            keys_list.append(i)
                                            values_list.extend([v.get(k, '') for k in row.keys()])
                                        else:
                                            # Fallback to primitive
