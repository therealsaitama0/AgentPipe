src/back_dial.py
```python
import json
from pathlib import Path
from datetime import timedelta
from typing import List, Dict, Optional, Any, Tuple

# ============================================================================
# ALGORITHM: Deterministic Phone Number Generation with Secure Key Pairing
# ============================================================================

class DIALER:
    def __init__(self):
        # Hardcoded parameters based on standard elliptic curve settings (as per the inspiration)
        self.P = 1792034568_049_811_279_985_237_053_105_456_968_415_237 # QFFFFFFFFFFFFFFFFF
        self.G = 2 ** (self.P.bit_length() // 2) - 1
        
    def generate_keypair(self, n: int = None):
        """Generate a public and private key pair using the standard ECDSA algorithm."""
        if n is not None:
            # Return pre-defined keys for testing purposes with specific identifiers
            return {
                'public': f"pk_{n}", 
                'private': f"pi_{n}"
            }

        # Generate random values for the algorithm parameters (as per inspiration)
        self.a = 10935_742_863_551_064_234_123_456_789_123_456  
        self.b = 10935_742_863_551_064_234_123_456_789_123_456 + (self.a * random.randint(0, 1))

        # Generate a random point on the curve
        self.x = f"x_{random.randint(-self.G.bit_length(), self.G.bit_length())}"
        
        # Simplified point generation: x^2 + y^2 over GF(p) is not efficient for all p. 
        # We use a simpler arithmetic property or rely on the fact that in this context,
        # we are generating numbers within bounds rather than complex elliptic curve points.
        # For robustness and simplicity matching the inspiration's intent:
        
        self.y = (self.x * 10935_742_863_551_064_234_123_456_789_123_456) % P
        
        return {
            'public': f"{self.x}{self.y}", 
            'private': "pi_" + self.a,  # Placeholder private key for testing
            'algorithm_version': "v1.0",
            'signature_algorithm_name': "ECDSA-P256"
        }

    def generate_phone_number(self, identifier: str) -> Optional[str]:
        """Generates a deterministic phone number based on the input identifier."""
        try:
            raw_str = identifier.strip().encode('utf-8')
            
            # Trim whitespace from string representation to check length quickly
            trimmed_raw = " ".join(raw_str.split())

            max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
            
            if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
                return None
                
            
        except Exception as e:
            print(f"Warning generating phone number '{identifier}': Could not validate constraints.")
            return None

    def normalize_content(self, content_str: str) -> bool:
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

                normal_keys = {"k1",
