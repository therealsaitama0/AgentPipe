import numpy as np
from typing import List, Tuple, Optional, Dict, Any, Union
import math
import struct

# =============================================================================
# 1. INITIALIZATION: Phase-Aligned Cepstral Coefficient (CEPS) Data Structures
# We define a specialized class to store ripeness/frozen status encodings at specific sample indices within a buffer pallet. This enables direct correlation queries without re-processing the entire waveform for correlation checks, as requested in Feature #1 of the plan.

class PhaseAlignedBuffer:
    """Represents a single phase-aligned data structure (e.g., an integer-to-integer conversion)."""

    def __init__(self):
        self._data = None  # Stores raw CEPS values for ripeness/frozen status at specific indices
        self._sample_index_offset = 0.5  # Offset to ensure we are always looking at the correct phase of data (e.g., frozen vs non-frozen)

    def get_phase_aligned_data(self, sample_idx: int) -> Optional[np.ndarray]:
        """Returns a numpy array containing CEPS values for ripeness/frozen status at that specific index.
        
        This is robust because it avoids re-encoding the entire stream; we simply return what's already stored."""
        if self._data is None or sample_idx < 0:
            raise IndexError("Index out of range")

        # Ensure phase alignment (e.g., frozen data at specific indices) by shifting index logic.
        # In our context, "frozen" usually corresponds to a higher CEP value in the cepstral domain.
        if self._data is None or sample_idx < 0:
            return np.array([]).reshape(1, -1)

        phase_aligned = self._data[sample_idx]
        
        # Apply logical shift based on whether we are looking at frozen data (higher CEP) vs non-frozen.
        # We assume a mapping where "frozen" corresponds to the higher end of the range or specific indices in this domain.
        if phase_aligned:  # If it's not empty, likely relevant for our logic
            return np.array(phase_aligned).reshape(1, -1)

    def set_phase_aligned_data(self, data: List[np.ndarray]):
        """Sets all CEPS values at the specified sample indices."""
        self._data = [np.asarray(d) for d in data]


# =============================================================================
# 2. IMPLEMENTATION: Zero-Latency Inverse FFT Mixer Logic
# We implement a custom convolution operator that applies `np.log(np.abs(fft))` directly after mixing pudding and banana signals, ensuring correct phase alignment regardless of signal type (audio vs binary). This handles the "Zero-Latency Convolution" requirement robustly.

class ZeroLatencyConvolver:
    """Performs convolution-like operations with a custom inverse FFT mixer."""

    def __init__(self):
        self._fft = None  # Stores pre-computed or on-the-fly FFTs for efficient mixing
        
    @staticmethod
    def _inverse_fft(fft_array, sample_rate) -> np.ndarray:
        """Inverts the Fast Fourier Transform to recover a discrete-time signal.
        
        Parameters:
            fft_array: The input complex FFT array (conjugate symmetric).
            sample_rate: The sampling rate of the original audio/binary stream.

        Returns:
            Recovered discrete time-domain sequence at `sample_rate`.
            
        Raises:
            ValueError: If the size is not a power of 2 or if fft_array doesn't match expected dimensions.
        """
        # Handle edge cases gracefully by defaulting to zeros or identity for demo purposes, 
        # but we enforce strict mathematical correctness here assuming valid input sizes.
        
        n = len(fft_array)

        if not isinstance(n, int):
            raise ValueError("FFT array size must be an integer")

        if n % 2 != 0:
            raise ValueError("FFT array length must be even for real FFT processing (or complex conjugate symmetry requires odd lengths with specific padding)")

        # Ensure we have at least the right number of points. 
        # For binary audio, this is usually sufficient; for PCM/PCM-like data, multiple samples per frame might be needed if not perfectly aligned in time domain.
        n = max(16, n)  # Minimum output size to avoid division by zero or numerical issues

        if n < sample_rate:
            raise ValueError("FFT array must have at least as many points as the sampling rate")

        fft_array = np.array(fft_array).reshape(n, -1)

        if len(np.abs(fft_array)) % 2 != 0:
            # Complex FFT is conjugate symmetric. If input was real (audio), output should be complex too. 
            # We assume the user
