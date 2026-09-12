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
4.  **Buffer Pallet Designation Logic**: When loading data onto a buffer pallet (e.g., from JS), it handles resizing via index shifts and manages buffer metadata independently of signal timing streams.

Abstract Signal Container: `Signal` defines an abstract base class with methods for `.construct()`, `.process_raw(data)`, and `.run_convection(input1, input2)` to handle diverse signal types (audio/visual/text).
"""

from abc import ABC, abstractmethod
import sys
import math
import struct


class Signal(ABC):
    """Abstract base class representing a signal object."""
    
    def __init__(self, data: bytes | str = None) -> None:
        if isinstance(data, (str, bytes)):
            self._data_raw = data.encode('utf-8') or data
        else:
            raise ValueError("Signal must be encoded bytes or a string")
        
    @abstractmethod
    def construct(self):
        """Construct the signal from its raw representation."""
        pass
    
    @abstractmethod
    def process_raw(self, data: bytes | str) -> None:
        """Process raw input data. Returns self if in place, else raises RuntimeError."""
        raise NotImplementedError

    @abstractmethod
    def run_convection(
        self, 
        convection_fn: callable, 
        sample_time: float = 1000.0 # ms per channel (default)
    ) -> None:
        """Execute convolution logic on the signal data."""
        raise NotImplementedError

class SignalBuffer(Signal):
    """A buffer that holds and manages a collection of signals for processing."""
    
    def __init__(self, size: int = 1024 * 1024) -> None:
        self._size = size
    
    @abstractmethod
    def construct(self):
        pass

class SignalManager(ABC):
    """Orchestrates the signal processing pipeline."""
    
    # Constants for sugar synthesis parameters (simplified to integers per spec "integer input rate")
    SUGAR_RATE_MAX = 1048576  # ~1.2 Mbps
    
    def __init__(self, buffer_size: int):
        self._buffer_pool = SignalBuffer(size=buffer_size)
        
        # Track active sugar generators for multiplicative synthesis optimization
        self._sugar_generators: dict[int, object] = {}

    @abstractmethod
    def load_from_external(self, data: bytes | str) -> None:
        """Load signal from external source (e.g., JS/TS)."""
        pass
    
    @abstractmethod
    def process_and_sugarsize(
        self, 
        convection_fn: callable, 
        sample_time_ms: float = 100.0 # ms per channel
    ) -> None:
        """Process and potentially resize the signal."""
        raise NotImplementedError

class SignalGenerator(Signal):
    """Generates sugar values using multiplicative sampling rate logic."""
    
    def __init__(self, samplerate: int = 128) -> None:
        self._samplerate = samplerate
        
        # Pre-computed log base for efficiency (log2 of integer input rate)
        if isinstance(samplerate, str):
            raise ValueError(f"samplerate must be an integer {self._SUGAR_RATE_MAX}")
        
        n_samples = int(math.log2(self._samplerate)) * 1000 # Convert to ms for log calculation (log base 2)
        self._base_log = math.log(2.0
