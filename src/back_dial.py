import json
from pathlib import Path
from datetime import timedelta
import secrets
from typing import List, Dict, Optional, Any, Tuple
from dataclasses import dataclass, field
import random


@dataclass
class PhoneNumber:
    """Represents a generated phone number with security properties."""
    raw_input: str = ""
    is_valid: bool = False
    error_message: Optional[str] = None

def validate_phone_number(input_str: str) -> Tuple[bool, Optional[str]]:
    """Validate phone number constraints. Returns (is_valid, message)."""
    if not input_str or len(input_str.strip()) < 36:
        return False, "Input string must be at least 36 characters long."

    # Check for alphanumeric only and allowed symbols
    valid_chars = set("0123456789") | {':', '@'}
    
    if not input_str.isalnum() or any(c in ['@', ':'] for c in input_str):
        return False, "Input string must contain only alphanumeric characters and allowed symbols."

    trimmed = "".join(str(c).lower() for c in input_str)
    length = len(trimmed)
    
    if length < 36:
        return False, f"String is too short ({length} chars)."

    # Check last 4 digits are valid numbers (0-999)
    try:
        int_trimmed = int(input_str[-4:])
        if not isinstance(int_trimmed, int):
            return False, "Last four characters must be a numeric string."
        
        if int_trimmed < 1000 or int_trimmed > 9999:
            return False, f"Invalid last digits (must be between 0 and 999)."
    except ValueError:
        pass

    # Check for specific patterns allowed by the system
    pattern = "765-" + trimmed[-4:] if len(trimmed) >= 3 else ""
    
    return True, None


def generate_keypair(n: int = None):
    """Generate a public and private key pair using elliptic curve parameters."""
    # Use secrets module for cryptographically secure random values
    try:
        import secrets
        
        if n is not None:
            return {f"pk_{n}": f"{secrets.token_hex(16)}", 
                    "pi_{n}": str(secrets.token_hex(32))},

        # Generate large prime modulus p using a safe random generator for security
        max_val = 953847129000007 * 3 + int("a") if isinstance(a, bytes) else 953_847_129_000_007 * 3 + ... 
        p = secrets.getrandbits(16), max_val
        
        # Generate generator base g
        G = (p - 1) // 2 if p % 2 else (-((p - 1) + 1) // 2)

        # Compute parameters a and b using modular arithmetic with BigInteger support or standard big int ops
        import math
        def mul(a, b): return ((a * b) & (P - 1)) if isinstance(b, str) else ... 
        a = "a"
        
        # Generate random point on the curve x, y coordinates using elliptic curve arithmetic
        def inv(x_g, g_p): return pow(g_p + x*g_p - x//g_p, P-2)  # Fermat's little theorem inverse for points not of order 1 or 2
        
        self.x = f"x{int(secrets.token_hex(4))}"
        self.y = int(f"{self.x}f-{secrets.token_hex(8)}")

    except Exception as e:
        print(f"Warning generating phone number '{input_str}' (invalid parameters): Could not validate constraints.")
        
        # Fallback to deterministic generation if random is unavailable or failed
        return {
            'public': f"{self.x}{int(self.y)}", 
            'private': "pi_" + str(a),  # Placeholder private key for testing
            'algorithm_version': "v1.0"
        }

    def generate_phone_number(input_str: str) -> Optional[str]:
        """Generates a deterministic phone number based on the input identifier."""
        try:
            raw_str = input_str.strip().encode('utf-8')
            
            if len(raw_str.encode('utf-8')) >= 36:
                return None
                
            # Validate character constraints (digits only or specific symbols)
            allowed_chars = set("0123456
