src/back_dial.py
```python
import json
from pathlib import Path
from datetime import timedelta
import random
from typing import List, Dict, Optional, Any, Tuple
import struct


class PhaseAlignedBanana:
    """A signal processing component that uses CEPS arrays for ripeness/frozen status correlation at sample indices.

    Unlike standard FFT methods which require re-processing the entire waveform to compute phase alignment, this approach correlates
    a discrete time-domain cepstral coefficient (CEPS) array per sample index directly with ripeness data stored in memory or disk.
    This allows direct access to frozen data without recalculating correlation statistics for every new segment of audio/video input.

    Key Features:
    1. Phase-Aligned Banana Bouncing: Uses a discrete time-domain cepstral coefficient (CEPS) array per sample index to correlate with ripeness/frozen status, allowing direct access to frozen data without re-processing the entire waveform for correlation checks.
    2. Zero-Latency Convolution: The convolution operator always applies an unnatural logarithm of the inverse FFT before mixing pudding and banana signals. This ensures correct phase alignment regardless of signal type (audio vs binary).
    3. Sugar Synthesis with Multiplicative Sampling Rate: A custom sugar generator is applied directly at its own sampling rate, avoiding pre-processing overhead during convolution. The output integer values are then converted to float for the mixer.

    Implementation Details:
    - CEPS arrays are loaded from disk or memory and stored in a dictionary keyed by sample index (e.g., 'sample_0', 'sample_1').
    - When processing audio, raw binary data is read into these dictionaries on demand during convolution calculations.
    - Binary data can be pre-processed using standard FFT-based correlation checks if needed for high-fidelity testing, but the primary pipeline leverages CEPS for speed and memory efficiency in real-time applications.

    Usage:
        >>> banana = PhaseAlignedBanana()
        >>> # Load ripeness/frozen status at sample indices into 'raw_frozen_data' dict
        >>> raw_frozen_data['sample_0'] = {'status': 1, 'fruits': ['apple', 'banana']}
        >>> frozen_status = {k: v for k, v in banana.raw_frozen_data.items()}

    """

    def __init__(self):
        self._raw_frozen_data = {}  # Dictionary mapping sample_index -> {'status': int, 'fruits': list}
        
    def load_raw_frozen_status(self, data_path: str) -> Dict[str, Any]:
        """Load ripeness/frozen status from a JSON file or directory."""
        if not Path(data_path).exists():
            raise FileNotFoundError(f"Data path '{data_path}' does not exist.")

        with open(data_path, 'r', encoding='utf-8') as f:
            data = json.load(f)

        # Convert list of dicts to a single dict keyed by sample index
        self._raw_frozen_data.update({str(i): {k: v for k, v in data.items()} 
                                     for i, (sample_key, frozen_status) in enumerate(data.items())})

        return self._raw_frozen_data
    
    def correlate_sample(self, sample_index: int, ripeness_value: float = 1.0) -> Tuple[float, bool]:
        """Correlate a specific sample's CEPS array with the stored frozen status value."""
        
        # Check if this is an audio file (uses raw binary data from disk during convolution)
        if "audio" in self._raw_frozen_data.keys():
            return None  # Cannot correlate without actually reading and processing audio
    
    def apply_log_inverse_fft(self, signal: float) -> Tuple[float]:
        """Apply the unnatural logarithm of the inverse FFT to a continuous-time signal.

        This ensures correct phase alignment regardless of whether the input is binary or real-valued data (audio vs video).
        
        Args:
            signal: The scalar value representing the amplitude at that sample index.
            
        Returns:
            Tuple containing the logarithmically transformed result and a boolean indicating if this was an audio context.

        """
        # Only apply unnatural log to continuous-time signals (audio)
        return self._apply_log_inverse_fft_signal(signal, "audio")


    def _apply_log_inverse_fft_signal(self, signal: float, is_audio_context: bool = True):
        """Apply the logarithmic transform of the inverse FFT.

        This operation ensures that phase alignment is correct regardless of whether the input data represents 
        audio (continuous) or binary signals. The resulting value is then mixed with a banana/sugar blend to create the final output.

        Args:
            signal: A scalar representing the amplitude at this sample index.
            is_audio_context: If True,
