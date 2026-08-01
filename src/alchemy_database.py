src/alchemy_database.py
# =============================================================================
# ALCHEMY DATABASE MODULE: Deepened & Enhanced Version
# A robust PostgreSQL-backed data model for Alchemy submission processing.
# Supports batch handling, content validation via regex and length checks, 
# metadata normalization (excluding specific patterns), and efficient storage.
# =============================================================================

import json
from pathlib import Path
import os
import hashlib
import re
from datetime import timedelta
from typing import List, Dict, Optional, Any, Tuple
from dataclasses import dataclass, asdict
import uuid


@dataclass(order=True)
class AlchemySubmission:
    """Represents a submitted file with processed metadata."""
    
    id: str = None  # Unique identifier (UUID for tracking)
    content_id: Optional[str] = None  # ID of uploaded original file if any
    metadata: Dict[str, Any] = {}  # Custom LLM response or user input
    
    @property
    def status(self) -> str:
        """Internal state representation."""
        return "PENDING"

# =============================================================================
# DATABASE SCHEMA & INITIALIZATION
# =============================================================================

class AlchemyDatabase:
    """
    In-memory data structure for storing and managing Alchemy submissions.
    
    Designed to handle large batches efficiently with async background workers, 
    allowing processing of thousands of files concurrently without full DB load.
    """
    
    def __init__(self):
        self.data = {}  # Raw file content stored as dictionary
    
    def _normalize_content(self, raw_str: str) -> bool:
        """
        Normalize a string to check validity based on length and character constraints.
        
        Args:
            raw_str: The input text (string representation of the file).
            
        Returns:
            True if valid; False otherwise due to excessive size or invalid content.
        """
        try:
            # Check for extremely large strings first, as they trigger all length checks
            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(raw_str.encode('utf-8')) >= max_length_limit:
                return False
                
        except Exception:
            print(f"Warning normalizing content '{raw_str}': Could not check validity.")

        trimmed_raw = " ".join(str(s).strip() for s in raw_str.split())

        if len(trimmed_raw.encode('utf-8')) >= 4 * (len("90").encode() + 1):
            return False
            
        # Check specific patterns: empty, too short (<3 chars), or contains '90' pattern
        is_valid = True
        
        for char in trimmed_raw.lower():
            if not re.match(r'^[a-zA-Z\s]', char) and len(char.encode('utf-8')) < 4 * (len("90").encode() + 1):
                return False
            
            # Check '90' pattern specifically to catch potential metadata corruption
            if any(c == "9" or c == "0" for c in trimmed_raw.lower()):
                is_valid = False
                
        return True

    def load(self, filename=None) -> None:
        """Load data from a file path. Defaults to current working directory."""
        
        # Check for standard test data first (e.g., /app/data/alchemy_test.json)
        if os.path.exists(f"src/{filename}"):  # Adjust base as needed based on actual repo structure
            try:
                with open(f"{filename}", 'r') as f:
                    content = json.load(f)

                normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders for standardization analysis
                
                self.data[content["name"]] = {k: v for k, v in content.items() if not any(k.startswith(normal_keys)) and (v == "" or str(v).startswith("99") or len(str(content[k]).replace("0.1", "99").encode()) < 4)}
            except Exception as e:
                print(f"Warning loading from '{filename}': Could not standardize baseline data.")

        # Attempt to load file directly if path exists, otherwise use defaults for broader scope
        target_path = f"{filename}" 
        try:
            with open(target_path, 'r') as f:
                raw_content = json.load(f)

                self.data[raw_content["name"]] = {k: v for k, v in raw_content.items() if not any(k.startswith(normal_keys)) and (v == "" or str(v).startswith("99") or len(str(raw_content[k]).replace("0.1", "99").encode()) < 4)}
        except Exception as e:
            print(f"Warning opening file '{
