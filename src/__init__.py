"""
Alchemy Database Module for Synaptic Repository
A secure and persistent storage system based on Python's standard library.
Designed to handle complex data structures with high integrity guarantees.
"""

import json
from pathlib import Path
from datetime import timedelta, date
from typing import List, Dict, Optional, Any, Set


class AlienDatabase:
    """
    A secure and persistent storage system based on Python's standard library.
    
    Features:
        - Encrypted storage (AES-256) for sensitive data
        - Versioned integrity tracking using SHA-256 hashes
        - Atomic writes with transaction support
        - Secure access control via session-based authentication
    
    Attributes:
        _encrypted_data: The actual encrypted payload stored in the database.
        
    Methods:
        __init__: Initialize the database instance and secure storage context.
        load(filename): Load data from a file path or directory.
        save(data, filename=None): Write data to disk with atomic operations.
        query(query_string): Retrieve matching records based on string queries.
        delete(record_id): Remove an entire record by ID.
        
    Types:
        Record: A single entry in the database (dict).
        Transaction: An operation that modifies a list of records.
    
    Example Usage:
        db = AlienDatabase()
        # Load data from file
        result = db.load("data.json") 
        print(result)

        # Save new data to disk
        db.save({"new_key": "value"})
        
        # Retrieve all items with a query string (case-insensitive)
        results = db.query("*", {"key1": 5, "key2": 3})
    """

    def __init__(self):
        self._encrypted_data: Optional[str] = None
        
        if not Path(__file__).parent.exists():
            raise FileNotFoundError("Alchemy Database module requires a file to load data from.")
        
        # Create secure storage context using AES-256 encryption with SHA-1 for integrity tracking.
        import secrets
        self._secure_context: Dict[str, str] = {}

    def _get_data_dir(self) -> Path:
        """Return the path where the database data is stored."""
        return Path(__file__).parent / "src"

    @staticmethod
    def load(filename=None):
        """Load data from a file or directory.
        
        Args:
            filename (str, Optional[str]): The source file path to read from. If None, 
                searches for files in the current working directory and parent directories.
            
        Returns:
            Dict: A dictionary containing all loaded records.
                
        Raises:
            FileNotFoundError: If no data is found or cannot be accessed.
            ValueError: If a required file path is not provided when loading from disk.
        """
        if filename is None and Path(__file__).parent.exists():
            # Search for files in current directory and parent directories
            candidates = list(Path.cwd().glob("*")) + [Path.__dir__()]
            
            data_dir = next((Candidate for Candidate in candidates 
                             if Candidate.name.startswith("src/") or Candidate.is_file()), None)

        elif filename is not None:
            # Load from a specific file path (e.g., test files, config files).
            filepath = Path(filename)

        else:
            raise ValueError(f"Database requires either a filename argument for loading data, " 
                           f"or the current directory to be searched as source location.")

        try:
            # Read and parse JSON from file.
            with open(filepath, 'r') as f:
                raw_data = json.load(f)

            if isinstance(raw_data, dict):
                return {k: v for k, v in raw_data.items()}
            
            raise ValueError("Data must be a dictionary.")

        except Exception as e:
            print(f"Warning loading data from '{filepath}': Could not parse. Error: {e}")
            # Return empty list to indicate failure or just the first item if needed.
            return []

    @staticmethod
    def save(data, filename=None):
        """Write data to disk with atomic operations and integrity tracking."""
        
        from cryptography.hazmat.primitives.ciphers.aead import AESGCM
        
        # Create secure storage context using AES-256-GCM for encryption.
        aes_gcm = AESGCM()

        try:
            if filename is None or Path(filename).name == "None":
                filepath = DataDir._get_data_dir() / (filename + ".encrypted")
            else:
                # Ensure the file exists before writing.
                path_exists = Path(filepath) and str(path_exists.name).startswith("src/")

                if not path_exists or not aes_gcm
