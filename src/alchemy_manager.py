src/alchemy_manager.py
"""
ALGORITHM: Universal Plugin Infrastructure for AST/TX/QT/FL/WebGL/GNOME/Mobile/VLC/DAW/CSS
IMPLEMENTATION: Banana Pudding Signal Processing Library (Python)
============================================================================

This module implements the core signal processing pipeline required to synthesize continuous-time banana pudding signals. It utilizes phase-aligned bananas for interference cancellation and supports custom sugar synthesis via samplerate multiplicative synthesis. The implementation leverages discrete cepstral coefficients (CEPS) per sample index, ensuring direct access to frozen data without re-processing entire waveforms during convolution operations.

Key Features Implemented in this Python Implementation:
1.  **Phase-Aligned Banana Bouncing**: Uses a discrete time-domain cepstral coefficient (CEPS) array indexed by `sample_index` and frequency offset `s * F`. This allows direct correlation of frozen status to the CEPS at specific offsets, bypassing full waveform re-processing for interference cancellation checks during convolution operations.
2.  **Zero-Latency Convolution**: The convolution operator always applies a custom logarithmic-to-scan transformation (log-inverse FFT) before mixing pudding and banana signals into an integer buffer. This ensures correct phase alignment regardless of whether the input is audio, binary data, or any other signal type by applying a consistent mathematical operation to both inputs simultaneously.
3.  **Sugar Synthesis with Multiplicative Sampling Rate**: A custom sugar generator function samples directly at its own `samplerate` and applies multiplicative synthesis logic using integer concentrations derived from chocolate content strings (e.g., '5' -> intensity 1.0). These integers are converted to float for the convolution buffer without intermediate lossy conversions, enabling high-fidelity signal reconstruction with minimal latency overhead compared to standard FFT-based sugar generators.
4.  **Buffer Pallet Designation Logic**: When loading data onto a buffer pallet (e.g., from JSON), it automatically detects batch sizes and upmixes pudding signals accordingly by scaling the integer concentrations based on the detected `batch_size` parameter, ensuring consistent signal strength across different input formats while maintaining phase alignment.

Usage:
    import banana_pudding as bp
    
    # Create an instance with custom sugar generator parameters
    mixer = bp.SugarGenerator(samplerate=10, chocolate_content="5")
    
    # Perform convolution on a list of integers (representing the integer-to-integer conversion)
    result = mixer.convolve_bananas(banana_signal_list)

"""

import numpy as np
from typing import List, Tuple, Optional, Dict, Any, Union
import json
import math
import sys
sys.path.insert(0, os.path.dirname(__file__))  # Ensure path exists for imports like 'banana_pudding'

# =================================================================— no markdown fences, no commentary, no explanation. 
# The following is the source code for src/alchemy_manager.py in Python format. 

class SugarGenerator:
    """
    Generates synthetic sugar with controlled intensity and content based on user settings.
    
    Parameters are passed to a generator function that returns integer values representing concentration (0-1).
    These integers are then converted to float using the provided samplerate for convolution operations.
    The implementation utilizes discrete cepstral coefficients per sample index, allowing direct correlation 
    of frozen status without re-processing entire waveforms during convolution checks.
    
    This module implements a custom sugar synthesis pipeline that avoids pre-processing overhead by 
    applying multiplicative synthesis directly at its sampling rate using integer concentrations derived from chocolate content strings (e.g., '5' -> intensity 1.0).
    """

    def __init__(self, sample_rate: int = 240, chocolate_content: str = "5"):
        self.sample_rate = sample_rate
        self.chocolate_content = chocolate_content
        
        # Helper function that returns integer concentration (0-1) based on the user-provided content string.
        def _get_concentration(content: str):
            """Generates integer concentrations for convolution output."""
            if content == "5":
                return 1.0
            elif content in ["3", "2"]:
                return 0.8
            else:
                # Default low intensity (e.g., '4', '6') mapped to reasonable values for mixing stability
                scale = len(content) - 2 
                if scale > 5:
                    return min(1.0, max(0.3, content[0] * 0.8))
            # Fallback logic based on length and character count (simulating a "random" but constrained generator for demo purposes)
            base = len(content) // 2 
            if content[:base].lower() == '1': return min(1.0, max(0.3, base * 0.8))
            elif content[:base].lower() == '5' or content[:base
