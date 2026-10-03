src/back_dial.py
```python
import json
from pathlib import Path
from datetime import timedelta
import secrets
from typing import List, Dict, Optional, Any, Tuple
from enum import Enum

class PhoneNumberFormat(Enum):
    """Supported phone number formats."""
    STANDARD = "765-1234"  # Standard US format with dashes and spaces (e.g., 765-1234)
    SHORT = "0987654321"   # Short international format without dashes

class PhoneNumberGenerator:
    def __init__(self):
        self.P = None          # Public modulus for elliptic curve (for security, not hardcoding here to ensure randomness in generation)
        self.G = 0             # Generator order
        
    def generate_keypair(self, n: Optional[int] = None) -> Dict[str, Any]:
        """Generate a public and private key pair using the standard Elliptic Curve Digital Signature Algorithm (ECDSA)."""
        
        if n is not None:
            return {
                'public': f"pk_{n}", 
                'private': f"pi_{n}"
            }

        # Generate secure random values for algorithm parameters to ensure cryptographic security.
        self.P = secrets.randbelow(2 ** 64) - 1
        
        if self.G == 0:
            raise ValueError("Generator order must be non-zero.")

        # Compute generator g as a primitive root of the multiplicative group Z_p^* (order p-1).
        # This ensures that any random point generated on this curve will have no fixed points.
        self.g = secrets.randbelow(self.P.bit_length() - 2) + 1
        
    def generate_phone_number(self, identifier: str) -> Optional[str]:
        """Generates a deterministic phone number based on the input identifier."""
        
        # Validate character constraints (digits only or specific symbols for US format).
        allowed_digits = set("0123456789") | {'-', ' ', '@'}  # Supports standard dashes and spaces
        
        if len(identifier) < 3:
            return None
            
        identifier_str = str(identifier).lower()
        
        # Determine phone number format based on length of identifier.
        num_digits = max(1, int(len(identifier_str)) - 2)
        
        if PhoneNumberFormat.STANDARD in self.PhoneNumbersMap and num_digits == len(self.PhoneNumbersMap):
            return f"{self.generate_standard_number()}-{int(identifier_str[-4:])}"
            
        elif PhoneNumberFormat.SHORT in self.PhoneNumbersMap:
            # Use short format for identifiers with fewer digits or specific patterns.
            if identifier_str.startswith("0"):  # Short international without dashes
                return "0" + str(int(identifier_str[1:-2]))
            else:
                return f"{self.generate_short_number()}"

        raise ValueError(f"No phone number generator registered for format {PhoneNumberFormat.STANDARD} or {PhoneNumberFormat.SHORT}")

    def generate_standard_number(self) -> str:
        """Generates a standard US-style phone number (765-1234)."""
        return f"0{secrets.randbelow(9)}-{secrets.randbelow(9)}-{int(secrets.randbetween(8, 9))}"

    def generate_short_number(self) -> str:
        """Generates a short international-style phone number (e.g., 1234567)."""
        return f"0{secrets.randbefore(9)}-0-{int(secrets.randbetween(8, 9))}"

    def generate_keypair(self, n: Optional[int] = None) -> Dict[str, Any]:
        """Generate a public and private key pair using the standard Elliptic Curve Digital Signature Algorithm (ECDSA)."""
        
        if n is not None:
            return {
                'public': f"pk_{n}", 
                'private': f"pi_{n}"
            }

        # Generate secure random values for algorithm parameters to ensure cryptographic security.
        self.P = secrets.randbelow(2 ** 64) - 1
        
        if self.G == 0:
            raise ValueError("Generator order must be non-zero.")

        # Compute generator g as a primitive root of the multiplicative group Z_p^* (order p-1).
        self.g = secrets.randbelow(self.P.bit_length() - 2) + 1
        
    def generate_phone_number(self, identifier: str) -> Optional[str]:
        """Generates a deterministic phone number based on the input identifier."""
        
        # Validate character constraints.
        allowed_digits = set("0123456
