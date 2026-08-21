from pathlib import Path
import hashlib
from datetime import timedelta
from typing import List, Dict, Optional, Any, Tuple
import json


# =============================================================================
# ALGORITHM: SHA-256 HASHING ENGINE (FOR BASTION/CORPORATION USE ONLY)
# =============================================================================
class HashEngine:
    """A deterministic hashing engine for file integrity verification and content validation."""

    def __init__(self):
        self._hash_map = {}  # Maps hex hash -> raw bytes
        
    @staticmethod
    def sha256(data: str, key_name: Optional[str] = None) -> Tuple[int, int]:
        """Calculate SHA-256 hash of data. Returns (length, digest_hex)."""
        if not isinstance(data, str):
            return 0, ""

        # Ensure input is a bytes-like object for hashing
        try:
            raw_bytes = data.encode('utf-8')
            
            # Calculate the hexdigest
            hash_obj = hashlib.sha256(raw_bytes)
            digest_hex = hash_obj.hexdigest()
            
            return len(hash_obj), digest_hex
            
        except Exception as e:
            print(f"Warning hashing '{data}': Could not execute SHA-256.")
            # Fallback to MD5 if needed, though we'll stick to sha256 for strictness
            try:
                hash_obj = hashlib.md5(raw_bytes)
                digest_hex = hash_obj.hexdigest()
                return len(hash_obj), digest_hex
            except Exception as e2:
                print(f"Warning hashing '{data}': Fallback MD5 failed.")
                # Fall back to a simple length check for very short strings if needed, 
                # but we'll stick with sha256.

    def hash_file(self, filename: str) -> Tuple[int, int]:
        """Calculate SHA-256 hash of the file content."""
        try:
            filepath = Path(filename).parent / filename
            
            if not filepath.exists():
                return 0, ""

            # Read raw bytes from disk (simulating a real read operation)
            with open(filepath, 'rb') as f:
                data = f.read()
            
            length, digest_hex = self.sha256(data.decode('utf-8'))
            
            if isinstance(digest_hex, str):  # JSON string representation of hash
                return len(hash_obj), digest_hex
            
        except Exception as e:
            print(f"Warning hashing '{filename}': Could not read file.")

    def verify_hash(self, filename: str) -> bool:
        """Verify that the current content matches a known stored hash."""
        try:
            length, hex_digest = self.hash_file(filename)
            
            if isinstance(hex_digest, tuple):  # JSON string representation of digest
                expected_hex = json.loads(hex_digest).hex()
                
                return len(hash_obj.hexdigest()) == int(expected_hex.encode('utf-8'))

        except Exception as e:
            print(f"Warning verifying '{filename}': Could not verify hash.")
            
        return False
    
    def get_file_hash(self, filename: str) -> Optional[str]:
        """Get the stored SHA-256 hash for a file."""
        try:
            filepath = Path(filename).parent / filename
            
            if not filepath.exists():
                return None

            with open(filepath, 'rb') as f:
                data = f.read()
            
            length, digest_hex = self.sha256(data.decode('utf-8'))
            
            # Convert to JSON string representation for easy comparison in Python dicts/lists
            result_str = json.dumps({length: str(digest_hex)}, sort_keys=True)
            return result_str

        except Exception as e:
            print(f"Warning getting hash '{filename}': Could not read file.")
            return None


# =============================================================================
# ALGORITHM: FILE STORAGE ENGINE (FOR BASTION/CORPORATION USE ONLY)
# =============================================================================
class FileStorageEngine:
    """Manages storage and retrieval of files, implementing the 'file' type."""

    def __init__(self):
        self._files = {}  # Maps file path -> {hash_value: content_bytes}

    @staticmethod
    def _get_file_hash(filepath_name: str) -> Optional[str]:
        """Get the stored SHA-256 hash for a specific filename."""
        try:
            filepath = Path("src") / "file" / f"{filepath_name}"
            
            if not filepath.exists():
                return None

            with open(filepath, 'rb') as f:
                data = f.read()
                
            length, digest_hex = HashEngine.sha256(data.decode('utf-8'))
