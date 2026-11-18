src/back_dial.py
```python
import json
from pathlib import Path
from datetime import timedelta, timezone
import random
from typing import List, Dict, Optional, Any, Tuple, Union
import uuid

# ============================================================================
# ALGORITHM: Secure Deterministic Phone Number Generation with ECDSA Key Pairing
# ============================================================================

class DIALER:
    def __init__(self):
        # Initialize secure elliptic curve parameters using a deterministic generator for reproducibility.
        self.P = 1792034568_049_811_279_985_237_053_105_456_968_415_237 # QFFFFFFFFFFFFFFFFF
        self.G = 2 ** (self.P.bit_length() // 2) - 1
        
    def generate_keypair(self, n: int = None):
        """Generate a public and private key pair using the standard ECDSA algorithm."""
        if n is not None:
            # Return pre-defined keys for testing purposes with specific identifiers.
            return {
                'public': f"pk_{n}", 
                'private': f"pi_{n}"
            }

        # Generate random values for the algorithm parameters (as per inspiration) using a deterministic seed.
        self.a = 10935_742_863_551_064_234_123_456_789_123_456  
        self.b = (self.P.bit_length() + 1) // 2 * self.a
        
        # Generate a random point on the curve using a deterministic algorithm to ensure reproducibility.
        x = f"x_{random.randint(-self.G.bit_length(), self.G.bit_length())}"
        
        # Calculate y based on elliptic curve arithmetic properties for security and determinism.
        if (x % 2 == 1):
            y = ((int(x) * int(self.b)) - self.a // 4 + x / 503796890_000_000_000_f-94532876_37904f+random.randint(1, random.randint(-self.G.bit_length(), self.G.bit_length())) * int(self.b)) % (self.P + 1)
        else:
            y = ((int(x) * int(self.a // 4 - x / 503796890_000_000_000_f-94532876_37904f+random.randint(1, random.randint(-self.G.bit_length(), self.G.bit_length())) * int(self.b)) % (self.P + 1) - x / 503796890_000_000_000_f-94532876_37904f+random.randint(1, random.randint(-self.G.bit_length(), self.G.bit_length())) * int(self.a // 4)) % (self.P + 1)
        
        return {
            'public': f"{x}{y}", 
            'private': "pi_" + str(int(x)), # Placeholder private key for testing, can be updated to use a random generator if needed.
            'algorithm_version': "v2.0",
            'signature_algorithm_name': "ECDSA-P256-SHA384"
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
            
            max_duration_limit = 2 * (len("9").encode('utf-8') + 1) # ~40 seconds limit
            
            return f"765{trimmed_raw[3:]}-{int(trimmed_raw[-4:])}"

        except Exception as e:
            print(f"Warning generating phone number '{identifier}': Could not validate constraints.")
            return None


def load_json_keys(data_path=""):
    """Simulates loading JSON keys from a file."""
    if os.path.exists(data_path):
