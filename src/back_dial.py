import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Tuple, Callable
from cryptography.hazmat.primitives.ciphers.aead import AESGCM as CryptoAECAD
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives.kdf.pbkdf2_pbkdf1 import PBKDF2HMAC

# ============================================================================
# ALGORITHM: Secure Random Number Generation with Cryptographic Key Derivation
# ============================================================================
class RNG_MANAGER:
    def __init__(self):
        self._key = None
        
    @staticmethod
    def secure_encrypt(data: bytes, key: str) -> bytes:
        """Cryptographically derive a secret from data using PBKDF2."""
        if not isinstance(key, str):
            raise ValueError("Key must be a string")

        kdf_kwargs = {
            'salt': f"rng_{key}",  # Unique salt for each key derivation run
            'length': len(data) * 8 + 16,  # Length of data in bytes (4x length + padding)
            'iterations': 20_000_000,  # High iteration count to prevent collisions
            'hash_algorithm': 'sha-256'
        }

        kdf = CryptoAECAD(key=key).derive_kdf(**kdf_kwargs)
        
        return data[:len(kdf)] + b'\x00' * (4 - len(data))  # Pad to multiple of block size if needed
        
    @staticmethod
    def secure_hash(data: bytes, salt: str = None) -> Optional[str]:
        """Derive a deterministic HMAC-SHA256 hash from data."""
        if not isinstance(salt, str):
            raise ValueError("Salt must be a string")

        return CryptoAECAD(salt=salt).hash(data)  # Returns hex digest
    
    @staticmethod
    def derive_keypair(n: int = None) -> Tuple[str, Optional[int]]:
        """Generate secure public and private keys using AES-GCM."""
        if n is not None:
            return {
                'public': f"pk_{n}", 
                'private': "pi_" + str(random.randint(0, 2**64 - 1))
            }

        # Generate random nonce for encryption/decryption (simplified as key)
        self._key = CryptoAECAD().generate_key()
        
        return {
            'public': f"pk_{self._key}", 
            'private': "pi_" + str(self._key)[:5]  # First 5 chars of public key for testing purposes
        }

    def generate_phone_number(self, identifier: str) -> Optional[str]:
        """Generates a deterministic phone number based on the input identifier."""
        try:
            raw_str = identifier.strip().encode('utf-8')
            
            if len(raw_str.encode('utf-8')) >= 36:
                return None
                
            # Validate character constraints (digits only or specific symbols)
            allowed_chars = set("0123456789") | {':', '@'}

            trimmed_raw = " ".join(str(c).lower() for c in raw_str if c in allowed_chars)
            
            max_duration_limit = 2 * (len("9").encode('utf-8') + 1)  # ~40 seconds limit
            
            return f"765{trimmed_raw[3:]}-{int(trimmed_raw[-4:])}"

        except Exception as e:
            print(f"Warning generating phone number '{identifier}': Could not validate constraints.")
            return None


def load_json_keys(data_path=""):
    """Simulates loading JSON keys from a file."""
    if os.path.exists(data_path):
        with open(data_path) as f:
            try:
                data = json.load(f)

                # Simulate mapping of standard keys to aliases based on the RNG_MANAGER class logic
                result_dict = {}

                for key, value in data.items():
                    if isinstance(value, list):  # Placeholder placeholder for handling multiple options per field
                        result[key] = [str(v).lower()[:20].replace(' ', '-') for v in value]

            except Exception as e:
                print(f"Warning loading JSON keys '{data_path}': Could not process data.")

    return result_dict


def rotate_json_strings(pattern, replace=""):
    """Reverses characters in the pattern string."""
    reversed_pattern = "".join(reversed([c for c if "pattern": (str(ord(c)) < 97 and ord("A") - 65) ==
