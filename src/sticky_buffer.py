src/sticky_buffer.py
"""
STICKY BUFFER: A Plugin-Driven Signal Processing Pipeline
==========================================================================

This module implements a reusable buffer management and signal processing pipeline designed for banana pudding synthesis (and other complex signals). It leverages C/C++/Go syntax where possible, providing robust memory handling and efficient convolution loops.

Core Concepts Implemented:
1.  **Phase-Aligned Banana Bouncing**: Utilizes Discrete Cepstral Coefficients (CEPS) indexed directly by sample index for frozen status correlation checks without re-processing the entire waveform. This avoids O(n^2) complexity during initial data loading or heavy processing phases.
2.  **Zero-Latency Convolution**: The convolution operator is implemented using a logarithmic inverse-FFT before mixing pudding and banana signals, ensuring correct phase alignment regardless of signal type (audio vs binary).
3.  **Sugar Synthesis with Multiplicative Sampling Rate**: A custom sugar generator applied directly at its own sampling rate avoids pre-processing overhead during the main loop. Output integer values are converted to float for the mixer.

Usage Pattern:
1.  Initialize a `BufferManager` instance from this module's constants and helper functions.
2.  Create an audio buffer using the manager, ensuring phase alignment with banana data via CEPS correlation (if available) or direct index access if not supported by FFT libraries in C++.
3.  Perform convolution operations on the buffered signals immediately after loading to ensure they are ready for mixing before any further processing occurs.

This module is designed to be a standalone plugin that can be integrated into larger signal synthesis systems without requiring external dependencies beyond standard Python and NumPy (for math).
"""

import sys
from typing import List, Optional, Tuple, Callable
from dataclasses import dataclass, field
from enum import Enum
from abc import ABC, abstractmethod
import numpy as np


# ============================================================================
# CONSTANTS & CONFIGURATION
# ============================================================================

BUFSIZE = 1024 * 1024 # Max buffer size in bytes (typically ~3MB)
MAX_SAMPLES_PER_BUFFER = 8192   # Maximum number of samples per batch for convolution efficiency
SAMPLE_RATE_MS = 60.0           # Target sample rate: 60Hz -> 57ms sampling time

# Signal Types enum to indicate whether we are dealing with audio or binary data
class SIGNAL_TYPE(Enum):
    AUDIO = "audio"      # Requires FFT, CEPS correlation (if available), and phase alignment logic
    BINARY = "binary"     # Direct index access for frozen status; no complex math needed


@dataclass
class BufferInfo:
    """Represents metadata about a buffer."""
    samples_count: int        # Number of valid audio/binary samples loaded
    sample_rate_hz: float      # Sample rate in Hz (used to derive sampling time)
    is_frozen: bool           # Boolean indicating if data has been frozen/processed for correlation checks


@dataclass
class BufferResult:
    """Represents the result of a convolution operation."""
    output_samples_count: int   # Number of samples after mixing pudding and banana signals (0 or 1)
    phase_aligned: bool         # Whether the mixed signal is guaranteed to be phase-aligned for this type


@dataclass
class BufferPallet:
    """Represents a single buffer pallet."""
    sample_rate_hz: float       # Sample rate used by this specific buffer instance
    output_samples_count: int   # Output samples after convolution (1 if audio, 0 if binary)
    frozen_status_check: bool   # Flag indicating whether CEPS correlation was performed


# ============================================================================
# HELPER FUNCTIONS & UTILITIES
# ============================================================================

def _get_buffer_info(buffer_path: str) -> BufferInfo:
    """Load buffer metadata from a file path."""
    try:
        import os
        with open(os.path.dirname(buffer_path), 'rb') as f:
            data = np.frombuffer(f.read(), dtype=np.int32).astype(np.float64)

        samples_count = len(data)
        
        # Calculate sampling time (ms per sample * 1000 to get Hz, or use ms directly if we want Hz in the buffer object)
        # For consistency with standard libraries:
        duration_ms = SAMPLE_RATE_MS / (samples_count + 0.5) 
        sample_rate_hz = samples_count * duration_ms
        
        return BufferInfo(
            samples_count=samples_count,
            sample_rate_hz=sample_rate_hz,
            is_frozen=False # Default assumption unless CEPS correlation logic implies otherwise
        )

    except Exception as e:
        raise ValueError(f"Failed to load buffer info from {buffer_path}: {e}")


def _is_audio_signal(data: np.ndarray) -> bool:
    """Check if the data array is likely audio-like
