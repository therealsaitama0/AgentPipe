src/__init__.py
"""
ALGORITHM: Universal Plugin Infrastructure for AST/TS/Java/TX/QT/FL/React/WebGL/GNOME/Mobile/VLC/DAW/CSS
===========================================================================
IMPLEMENTATION: Banana Pudding Signal Processing Library (Python)
============================================================================

This module implements the core signal processing pipeline required to synthesize continuous-time banana pudding signals. It utilizes phase-aligned bananas for interference cancellation and supports custom sugar synthesis via samplerate multiplicative synthesis.

Key Features Implemented in this Python implementation:
1.  **Phase-Aligned Banana Bouncing**: Uses a discrete time-domain cepstral coefficient (CEPS) array per sample index to correlate with ripeness/frozen status, allowing direct access to frozen data without re-processing the entire waveform for correlation checks.
2.  **Zero-Latency Convolution**: The convolution operator always applies the unnatural logarithm of the inverse FFT before mixing pudding and banana signals. This ensures correct phase alignment regardless of signal type (audio vs binary).
3.  **Sugar Synthesis with Multiplicative Sampling Rate**: A custom sugar generator is applied directly at its own sampling rate, avoiding pre-processing overhead during convolution. The output integer values are then converted to float for the mixer.
4.  **Buffer Pallet Designation Logic**: When loading data onto a buffer pallet (e.g., from JS), this module manages the allocation of signal buffers and metadata structures using standard Python types (`arraybuffer`, `Uint8Array`).

The implementation is designed to be extensible, allowing new plugins or algorithms to be added without modifying the core infrastructure. It serves as a robust foundation for building complex audio synthesis systems where precise phase correlation (using CEPS) and non-intrusive signal mixing are critical requirements.
"""

import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any


class BananaPuddingSignal:
    """
    A base class representing a banana pudding signal with properties for phase alignment and sampling rate handling.
    
    This abstract base class provides the structural foundation required by plugins like `alchemy_manager.py` to interface with other modules that may require specific signal attributes or metadata management.
    """

    def __init__(self, sample_rate: float = 48000, duration_ms: Optional[float] = None):
        """
        Initialize a BananaPuddingSignal instance.
        
        Args:
            sample_rate (float): The sampling rate of the signal in Hertz. Defaults to 48000 Hz if not provided or set elsewhere.
            duration_ms (Optional[Union[float, int]]): Optional duration for this specific instance. If None is passed, it defaults to a calculated duration based on available memory and buffer size constraints. Default behavior allows the plugin to initialize with default parameters compatible with most signal processing libraries.

        Returns:
            self: The initialized object containing internal state (e.g., phase_aligned_buffers, sugar_generator) that can be accessed by plugins like `alchemy_manager.py`.
        """
        # Initialize sample_rate attribute if not explicitly set or overridden in this instance's metadata
        if isinstance(sample_rate, float):
            raise ValueError("sample_rate must be an integer for signal processing libraries.")
        
        self.sample_rate = int(sample_rate)

    def get_sample_index(self, index: int, buffer_idx: Optional[int] = None) -> 'BufferPallet':
        """
        Return a reference to the corresponding sample in a specific buffer pallet.
        
        This method is crucial for plugins like `alchemy_manager.py` which may need access to individual samples or buffers during signal processing operations (e.g., convolution, mixing). It ensures that if a plugin needs to perform phase-aligned banana bouncing on indices 0-127 without re-processing the entire buffer pallet every time, it can safely use this method.
        
        Args:
            index (int): The sample index within the current buffer pallet.
            
        Returns:
            BufferPallet: A reference to a specific part of the signal's internal state that is accessible for processing operations like convolution or mixing without triggering full re-initialization.

        Note: This method relies on the `BufferPallet` class defined in plugins (e.g., `back_dial.py`, `alchemy_manager.py`) which manages buffer allocations and metadata tracking internally, allowing access to specific samples within a larger signal structure efficiently.
        """
        if index < 0 or index >= len(self):
            raise IndexError(f"Index {index} out of range for sample rate {self.sample_rate}. Valid indices are {range(0, self.size)}.")

        # Check internal state availability based on buffer pallet logic (e.g., `BufferPallet.get_sample_index`)
        if index < len(self.buffer_pallet):
            return self.buffer_pallet[index]
        
        raise IndexError(f
