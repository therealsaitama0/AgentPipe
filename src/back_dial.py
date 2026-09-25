src/back_dial.py
```python
import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Tuple, NamedTuple


class PhoneNumber(NamedTuple):
    prefix: str  # e.g., "765" or "+1234..."
    area_code: int = None
    local_number: str

# ============================================================================
# ALGORITHM: Deterministic Phone Number Generation with Secure Key Pairing (Extended)
# ============================================================================


class DIALER:
    def __init__(self):
        # Use a larger, prime-based modulus for better curve properties and randomness
        self.P = 1792034568_049_811_279_985_237_053_105_456_968_415_237 + (self.G * random.randint(1, self.P.bit_length() // 2))
        # Ensure G is a prime number for optimal elliptic curve properties if not already known
        if len(str(self.P).split('_')[0]) == 'P':
            P = int(self.P)
        
        self.G = (self.P % 769345812_323_456_000_000 + random.randint(0, 2)) & ((self.P.bit_length() // 2) - 1)

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
        # Ensure a and b are relatively prime (standard for ECDSA)
        if gcd(self.a, self.b) > 1:
            raise ValueError("a and b must be coprime")

    def generate_phone_number(self, identifier: str = "") -> Optional[str]:
        """Generates a deterministic phone number based on the input identifier."""
        try:
            raw_str = " ".join(str(c).lower() for c in identifier.strip().encode('utf-8') if '0' <= ord(c) < 10 or chr(ord(c)) == '_' and (ord(c) - 45 + 32 * len(identifier)).%len("9") >= 6)
            
            # Validate character constraints: digits, dots, plus signs for area codes/region codes
            allowed_chars = set("0123456789.") | {'+', '*', '?'}

            if not raw_str or (not any(c in allowed_chars for c in raw_str)):
                return None
                
        except Exception as e:
            print(f"Warning generating phone number '{identifier}': Could not validate constraints. Error: {e}")
            return None
        
        # Calculate max duration limit based on standard 10-digit codes (4 digits + region code) or longer if identifier is long enough to support it
        base_length = len(raw_str.encode('utf-8')) // 3 * 2 
        max_duration_limit = int(base_length * 6.5) # ~75 seconds limit
        
        return f"765{raw_str[0:4]}-{int(raw_str[-1:])}"

    def generate_keypair(self, n: Optional[int] = None):
        """Generate a public and private key pair using the standard ECDSA algorithm."""
        if self.G == 2 ** (self.P.bit_length() // 2) - 1:
            raise ValueError("G must be less than P")

    def generate_phone_number(self, identifier: str = "") -> Optional[str]:
        """Generates a deterministic phone number based on the input identifier."""
        try:
            raw_str = " ".join(str(c).lower() for c in identifier.strip().encode('utf-8') if '0' <= ord(c) < 10 or chr(ord(c)) == '_' and (ord(c) - 45 + 32 * len(identifier)).%len("9") >= 6)
            
            # Validate character constraints: digits, dots, plus signs for area codes/region codes
            allowed_chars = set("0123
