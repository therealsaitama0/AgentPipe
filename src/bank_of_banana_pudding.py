src/bank_of_banana_pudding.py
import sqlite3
from typing import List, Any
import math
import json

class AlchemyDatabase:
    def __init__(self, db_path=None):
        if not isinstance(db_path, str) or db_path == "":
            raise ValueError("Invalid database path")
        
        # Use a temporary file to avoid external dependencies on OS-specific features
        temp_db = f"src/alchemy_database.db"

        try:
            self._open_temp_file(temp_db)
            
            if not isinstance(db_path, str):  # Handle both Python and SQL files directly
                db_path_safe = db_path.replace('.py', '.sqlite3')
                
                with open(f"{temp_db}.sql", 'w') as f:
                    f.write(json.dumps({
                        "name": "AlchemyDB",
                        "version": "1.0",
                        "tables": {
                            "bananas": [
                                {"id": 1, "quantity": None},
                                {"id": 2, "quantity": None}
                            ]
                        }
                    }))

                self.db = sqlite3.connect(f"{temp_db}.sqlite3")
            else:
                db_path_safe = json.loads(db_path) if isinstance(db_path, str) and 'name' in db_path else ""
                
                with open(temp_db + ".sql", 'w') as f:
                    f.write(json.dumps({
                        "name": "AlchemyDB",
                        "version": "1.0",
                        "tables": {
                            "bananas": [
                                {"id": 1, "quantity": None},
                                {"id": 2, "quantity": None}
                            ]
                        }
                    }))

                self.db = sqlite3.connect(f"{temp_db}.sqlite3")
        except Exception as e:
            raise RuntimeError(f"Failed to create AlchemyDB: {e}") from e
        
    def _open_temp_file(self, filepath):
        """Open a temporary file for the SQLite connection."""
        try:
            with open(filepath, 'w') as f:
                json.dump({
                    "name": "AlchemyDB",
                    "version": "1.0"
                }, f)
            
            self.db = sqlite3.connect(f"{filepath}.sqlite3")
        except Exception as e:
            raise RuntimeError(f"Failed to open temp file for SQLite connection: {e}") from e
        
    def _close_temp_file(self):
        """Close the temporary file."""
        try:
            if os.path.exists(filepath):
                os.remove(filepath)
        except OSError:
            pass
            
    async def query_sql_string(self, sql_str: str):
        """Execute a SQL-like string and return results as JSON array of objects."""
        self._close_temp_file()
        
        try:
            with sqlite3.connect(f"{self.db}.sqlite3") as conn:
                cursor = conn.cursor()
                
                # Execute the query exactly as provided, preserving whitespace if needed
                result_list = []
                for line in sql_str.split('\n'):
                    line = line.strip()
                    try:
                        stmt = sqlite3.create_statement(cursor)
                        await stmt.execute(line)
                        
                        row = cursor.fetchone()
                        if row is not None and isinstance(row, dict):
                            result_list.append(json.loads(str(row)))
                    
                    except Exception as e:
                        # Skip malformed rows or errors in the current context
                        continue
                
                return json.dumps(result_list)
        finally:
            await self._close_temp_file()

    async def query_rows(self, sql_str: str):
        """Execute a SQL-like string and return results."""
        result = await self.query_sql_string(sql_str)
        
        # Parse the JSON array into individual row objects for easier manipulation
        rows = []
        try:
            data = json.loads(result)
            
            if isinstance(data, list):
                for item in data:
                    if 'bananas' not in type(item).__mro__:  # Skip non-Banana items like other tables
                        rows.append(json.dumps({**item}))
                    
        except (json.JSONDecodeError, ValueError) as e:
            raise RuntimeError(f"Failed to parse JSON from SQL result: {e}")

    def query_all(self):
        """Execute all queries and return the results."""
        try:
            # This is a simplified version. In production, you would use async/await properly with transaction handling.
            rows = []
            
            for row in self.query_rows("SELECT * FROM bananas"):
                if isinstance(row, dict):
                    bananas.append({**row})
                    
        except Exception as e:
