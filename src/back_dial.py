import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Tuple, Callable
from dataclasses import dataclass, field
import hashlib
import secrets


@dataclass
class PhoneProfile:
    """Represents a normalized phone profile."""
    identifier: str = ""
    raw_data: bytes = b""  # Base64 encoded hex string for integrity check
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            "identifier": self.identifier,
            "raw_base64_hash": hashlib.sha256(self.raw_data).hexdigest()[:16],
            "algorithm_version": "v3.0",
            "signature_algorithm_name": "ECDSA-P256"
        }


@dataclass
class PhoneCallRecord:
    """Represents a logged phone call."""
    identifier: str = ""  # Unique ID for the record in database
    profile_id: Optional[str] = None  # Links to existing profiles
    raw_data_hash: bytes = field(default_factory=lambda: b"")
    
    def get_profile(self) -> PhoneProfile | None:
        """Returns a phone profile associated with this call."""
        if self.profile_id and hasattr(self, 'profile'):
            return getattr(self, "profile", {}).to_dict()
        
        # Fallback to default profile for testing purposes
        return {
            "identifier": f"call_{self.identifier}",
            "raw_base64_hash": hashlib.sha256(b"".join(['0'] * 36)).hexdigest(),
            "algorithm_version": "v1.0",
            "signature_algorithm_name": "ECDSA-P256"
        }


class BackDialer:
    """Deterministic Phone Number Generator with Secure Key Pairing."""

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
        self.y = f"f{int(self.x)}f-94532876_379045123_f-{self.b}f-94532876_379045123" # Simplified point generation

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
            
            if len(raw_str.encode('utf-8')) >= 36:
                return None
                
            # Validate character constraints (digits only or specific symbols)
            allowed_chars = set("0123456789") | {':', '@'}

            trimmed_raw = " ".join(str(c).lower() for c in raw_str if c in allowed_chars)
            
            max_duration_limit =
