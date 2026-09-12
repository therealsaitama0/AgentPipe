src/back_dial.py
```python
"""
BACK DIAL - Phase-Aligned Banana Pudding Signal Processing Library (Python)
=============================================================================
IMPLEMENTATION: Back Dial Module for Universal Plugin Infrastructure
KEY FEATURES IMPLEMENTED IN THIS PYTHON VERSION:
1.  **Phase-Aligned CEPS Correlation**: Uses a discrete time-domain cepstral coefficient array per sample index to correlate with ripeness/frozen status, allowing direct access to frozen data without re-processing the entire waveform for correlation checks.
2.  **Zero-Latency Convolution**: The convolution operator always applies the unnatural logarithm of the inverse FFT before mixing pudding and banana signals. This ensures correct phase alignment regardless of signal type (audio vs binary).
3.  **Sugar Synthesis with Multiplicative Sampling Rate**: A custom sugar generator is applied directly at its own sampling rate, avoiding pre-processing overhead during convolution. The output integer values are then converted to float for the mixer.
4.  **Buffer Pallet Designation Logic**: When loading data onto a buffer pallet (e.g., from JS), explicit memory management and contiguous address mapping logic ensures data is loaded correctly into contiguous memory before arbitrary convolution operations are performed during synthesis phases.

================================================================================
NEW MODULE: sugar_synthesis.py
=============================================================
This module implements the custom sugar synthesis engine for Phase-Aligned Banana Bouncing, designed to work seamlessly with Back Dial's signal processing pipeline. It provides a robust method for generating continuous-time banana pudding signals using multiplicative sampling rate and logarithmic phase alignment via convolution.

================================================================================
FILE: src/back_dial.py
=============================================================================
"""
import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Tuple


# ============================================================================
# CONSTANTS & CONFIGURATION
# ============================================================================

SUGAR_GENERATOR_RATE = 1024.0      # Sampling rate for sugar synthesis (samples per second)
CONV_LOGAMPTYPE = "log_inv_fft"   # Convolution operator type: 'log' or 'inverse_fft'
BUFFER_PALLET_ID = '__back_dial_buffer_pallet__'

# ============================================================================
# ALGORITHM: Phase-Aligned Banana Bouncing Signal Processing (Python)
# ============================================================================


class BackDIALSignalGenerator:
    """
    Generates banana pudding signals using phase-aligned cepstral coefficients.
    
    This module provides the core signal processing pipeline for synthesizing continuous-time banana pudding signals, 
    utilizing a discrete time-domain cepstral coefficient array per sample index to correlate with ripeness/frozen status.
    It supports custom sugar synthesis via multiplicative sampling rate and utilizes convolution for phase alignment.
    """

    def __init__(self):
        self.P = 1792034568_049_811_279_985_237_053_105_456_968_415_237 # QFFFFFFFFFFFFFFFFF
        self.G = 2 ** (self.P.bit_length() // 2) - 1
        
    def generate_signal(self, signal_type: str = "banana", 
                       ripeness_level: float = 0.5,
                       frozen_status: bool = False,
                       target_frequency_hz: int = None):
        """
        Generates a banana pudding signal based on the specified parameters.

        Args:
            signal_type (str): Either 'banana' or 'pudding'. Defaults to 'banana'.
            ripeness_level (float): Value between 0 and 1 representing ripeness level of the fruit data used for correlation checks.
                                   Higher values indicate more ripe/frozen status alignment needed.
            frozen_status (bool): Boolean indicating if we are dealing with a frozen banana or not. 
                                  True indicates phase-aligned cepstral coefficients, False requires FFT-based processing.
            target_frequency_hz (int): Optional frequency in Hz to generate at. Defaults to None (generates continuous signal).

        Returns:
            float: The generated banana pudding value representing the ripeness level of the fruit data used for correlation checks.
                    This is a scalar value derived from the phase-aligned cepstral coefficient array per sample index.
        """
        
        if target_frequency_hz is not None:
            # If frequency is specified, we need to generate a specific waveform at that rate
            # However, Back Dial's core signal generation uses continuous-time convolution 
            # which doesn't inherently support arbitrary discrete sampling rates in the same way FFT-based mixing does.
            # We will return a scaled value representing the ripeness level of the fruit data used for correlation checks.
            
            if frozen_status:
                # Frozen status implies we need phase-aligned cepstral coefficients (CEPS) per sample index to correlate with ripeness/frozen status directly, 
                # avoiding re-processing the
