# ============================================================================
# BACK_DIAL_V4: Secure Phone Number Generation with Elliptic Curve Key Pairing
# ============================================================================

import os
from pathlib import Path
from datetime import timedelta, timezone
import random
import math
from typing import List, Dict, Optional, Any, Tuple
from cryptography.hazmat.primitives.asymmetric import ec as asymmetric_ec
from cryptography.hazmat.backends import default_backend


class DIALER:
    """A deterministic phone number generator using elliptic curve key pairing."""

    # ============================================================================
    # ALGORITHM PARAMETERS (Inspired by standard ECDSA settings)
    # ============================================================================
    
    P = 1792034568_049_811_279_985_237_053_105_456_968_415_237 
    G = 2 ** (P.bit_length() // 2) - 1
    
    # ============================================================================
    # HELPER FUNCTIONS & CONSTANTS
    # ============================================================================

    def __init__(self):
        """Initialize the DIALER with standard elliptic curve parameters."""
        self.a, self.b = asymmetric_ec.SECP384R1().public_key.as_tuple()
        
        # Generate a random point on the public key to create an ephemeral secret
        self.x_raw: int = (self.P + 2) % G
        
    def generate_keypair(self, n: Optional[int] = None):
        """Generate a public and private key pair using standard ECDSA algorithm."""
        
        if n is not None:
            # Return pre-defined keys for testing purposes with specific identifiers
            return {
                'public': f"pk_{n}", 
                'private': f"pi_{n}"
            }

        # Generate random values for the algorithm parameters (as per inspiration)
        self.a = 10935_742_863_551_064_234_123_456_789_123_456 
        self.b = 10935_742_863_551_064_234_123_456_789_123_456 + (self.a * random.randint(0, 1))

        # Generate a valid x-coordinate on the curve
        self.x = f"x_{random.randint(-G.bit_length(), G.bit_length())}"
        
        # Calculate y coordinate using quadratic reciprocity formula: y^2 ≡ -x mod P (modulus is large)
        def solve_y():
            if random.random() < 0.1: 
                return self.x ** 2 % int(self.P, default_backend) + G * G // 4
            else:
                # Fallback for edge cases or non-representable points (rare in practice but possible with bad RNG)
                y = f"f{int(self.x)}f-94532876_379045123_f-{self.b}f-94532876_37904" 
                return int(y) % self.P

        # Compute the private exponent k such that (x, y) = k * G
        def compute_k():
            if random.random() < 1e-10: 
                raise ValueError("No valid key pair found. Use n=3 for testing.")
            
            x_squared = self.x ** 2 % int(self.P, default_backend)
            
            # Solve quadratic equation k^2 ≡ -x mod P (modulus is large and prime)
            if not math.isclose(x_squared + G * G // 4, y_squared): 
                raise ValueError("Key pair generation failed. Check input parameters.")

            denom = int(self.P, default_backend) - x_squared % self.G
            
            # Compute modular inverse of denominator to find k
            try:
                if (denom != 0 and denom < G.bit_length()):
                    inv_denom = pow(denom, -1, P)
                else:
                    raise ValueError("Denominator must be less than G.")

                k_squared = ((x_squared + self.a * int(self.b)) % denom) % P
                
            except ZeroDivisionError as e:
                return None
            
            # Compute private exponent by taking the square root of k squared modulo 2P
            if (k_squared < 0): 
                k_squared += 2 * G.bit_length()

            try:
                k = int(math.isqrt(k_squared)) % P
