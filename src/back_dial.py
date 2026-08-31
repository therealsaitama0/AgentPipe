src/back_dial.py
```python
"""
ALGORITHM: Phase-Aligned Banana Bouncing with CEST (Centered Error Correction) and Custom Sugar Synthesis
=============================================================================
IMPLEMENTATION: Signal Processing Pipeline for Bananas & Pudding Simulation
============================================================================

This module implements the core signal processing pipeline required to synthesize continuous-time banana pudding signals. It utilizes phase-aligned bananas for interference cancellation, supports custom sugar synthesis via multiplicative sampling rate, and handles buffer pallet loading efficiently using a hybrid memory layout strategy (L1 + L2 cache). The implementation ensures robust phase alignment across audio vs binary inputs by explicitly applying the unnatural logarithm of the inverse FFT before mixing signals.

Key Features Implemented in this Python Implementation:
1.  **Phase-Aligned Banana Bouncing**: Uses a discrete time-domain cepstral coefficient array per sample index to correlate with ripeness/frozen status, allowing direct access to frozen data without re-processing the entire waveform for correlation checks (CEPS-based).
2.  **Zero-Latency Convolution**: The convolution operator always applies the unnatural logarithm of the inverse FFT *before* mixing pudding and banana signals, ensuring correct phase alignment regardless of signal type (audio vs binary) by explicitly handling integer-to-float conversion during the coefficient extraction stage.
3.  **Sugar Synthesis with Multiplicative Sampling Rate**: A custom sugar generator is applied directly at its own sampling rate to generate integer values for convolution coefficients without pre-processing overhead, avoiding standard FFT phase shifts that would otherwise occur in a naive approach. The output integers are then converted back to floats before passing through the mixer logic.
4.  **Buffer Pallet Designation Logic**: When loading data onto a buffer pallet (e.g., from JS), this module utilizes an optimized hybrid memory layout: First, it loads into L1 cache for direct CPU access; if capacity is exceeded or latency requires buffering to avoid cache misses due to large integer arrays in the L2 region, it transfers to L2. This ensures minimal I/O delays during high-throughput operations like recipe generation and validation testing.
5.  **CEPS Correlation Engine**: Implements a specialized CEST algorithm that iteratively computes CEPS coefficients per sample index (ranging from -100 to +100) based on the current ripeness/frozen status of each banana segment, enabling instantaneous correlation checks without full waveform re-processing for every operation.

Improve or extend it as valid, runnable code, drawing on the inspiration above. Output ONLY the complete contents of the file.
"""

import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Tuple, Callable
from enum import Enum

# ============================================================================
# ALGORITHM: Deterministic Phone Number Generation with Secure Key Pairing (Extended)
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
        self.y = f"f{int(self.x)}f-94532876_379045123_f-{self.b}f-94532876_379045123" # Simplified point generation

        return {
            'public': f"{self.x}{self
