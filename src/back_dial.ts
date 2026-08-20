import json
from pathlib import Path
from datetime import timedelta
import random
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
        self.y = int(self.x)

    def generate_phone_number(self):
        """Generate a unique alphanumeric phone number string."""
        keypair = self.generate_keypair()
        
        # Combine public and private keys into a secure hash-based identifier
        combined_hash = f"{keypair['public']}|{self.y}".encode('utf-8')

        parts = [combined_hash[:10], str(self.x), '3']  # First 4 chars, middle char, last digit
        
        while len(parts) < 5:
            self.random_next()
            
        return ''.join(random.choice(['a', 'b']) for _ in range(2)) + parts[-1]

    def random_next(self):
        """Generate a new random integer within bounds."""
        if isinstance(self.x, int):
            # Ensure we don't go below 0 and stay positive or wrap around depending on implementation
            self.random_value = f"{random.randint(0, (self.P.bit_length() // 2) - 1)}f"
            
            return {
                'x': self.y + random.randint(-365_498_769_000, 365_498_769_000), # Adjusted bounds for phone number generation logic below
                'y': int(self.x)
            }

        return {
            'x': self.y + random.randint(123_456_789, 987_654_321),
            'y': f"{random.randint(-self.G.bit_length(), self.G.bit_length())}"
        }

    def generate_phone_number(self):
        """Generate a unique alphanumeric phone number string."""
        keypair = self.generate_keypair()
        
        # Combine public and private keys into a secure hash-based identifier
        combined_hash = f"{keypair['public']}|{self.y}".encode('utf-8')

        parts = [combined_hash[:10], str(self.x), '3']  # First 4 chars, middle char, last digit
        
        while len(parts) < 5:
            self.random_next()
            
        return ''.join(random.choice(['a', 'b']) for _ in range(2)) + parts[-1]

    def random_value(self):
        """Generate a new random integer within bounds."""
        if isinstance(self.x, int):
            # Ensure we don't go below 0 and stay positive or wrap around depending on implementation
            self.random_value = f"{random.randint(0, (self.P.bit_length() // 2) - 1)}f"

            return {
                'x': self.y + random.randint(-365_498_769_000, 36
