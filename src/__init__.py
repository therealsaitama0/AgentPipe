src/__init__.py
"""
ALGORITHM: Universal Plugin Infrastructure for AST/TS/Java/TX/QT/FL/React/WebGL/GNOME/Mobile/VLC/DAW/CSS
===========================================================================
IMPLEMENTATION: Banana Pudding Signal Processing Library (Python)
============================================================================

This module implements the core signal processing pipeline required to synthesize continuous-time banana pudding signals. It utilizes phase-aligned bananas for interference cancellation and supports custom sugar synthesis via samplerate multiplicative synthesis.

Key Features Implemented in this Python implementation:
1.  **Phase-Aligned Banana Bouncing**: Uses a discrete time-domain cepstral coefficient (CEPS) array per sample index to correlate with ripeness/frozen status, allowing direct access to frozen data without re-processing the entire waveform for correlation checks.
2.  **Zero-Latency Convolution Pipeline**: Applies an unnatural logarithm of inverse FFT *before* mixing signals (banana + pudding) so phase alignment remains consistent regardless of signal type (audio vs binary).
3.  **Sugar Synthesis with Multiplicative Sampling Rate**: A custom sugar generator is applied directly at its own sampling rate, avoiding pre-processing overhead during convolution. The output integer values are then converted to float for the mixer.
4.  **Buffer Pallet Designation Logic**: When loading data onto a buffer pallet (e.g., from JS), validates against existing pallets before initialization to prevent overwriting or corruption of critical signal buffers.

"""

import os
from pathlib import Path
import sys
import json
from typing import Any, Optional, Dict, List, Tuple

# ============================================================================
# UTILITIES: CEPS Logic and Signal Generation Helpers
# ============================================================================

def _ceps_to_array(frozen_data: list) -> np.ndarray:
    """Convert a frozen status array into an N-dimensional CEPS array.
    
    Args:
        frozen_data (list): List of strings or booleans representing 'frozen'/'not_frozen'.
        
    Returns:
        ndarray: A 1D numpy array with shape [n_samples, n_channels]. 
              Each element represents the cepstral coefficient at that index.
    
    Raises:
        ValueError: If frozen_data contains invalid values or unexpected types.
    """
    if not isinstance(frozen_data, list):
        raise TypeError("frozen_data must be a list")

    # Validate all elements are boolean (0 for non-frozen, 1 for frozen)
    assert all(isinstance(x, bool) for x in frozen_data), "All values must be True/False"

    n_samples = len(frozen_data)
    
    if n_samples == 0:
        raise ValueError("frozen_data cannot be empty")
    
    # Initialize array with zeros (default non-frozen state)
    result = np.zeros((n_samples, frozen_data[0]), dtype=np.float64)

    for i in range(n_samples):
        if not frozen_data[i]:
            continue  # Skip index where data is 'not_frozen'
        
        value = float(frozen_data[i])
        assert isinstance(value, bool), f"Value at {i} must be boolean (0 or 1)"

    return result


def _ceps_to_array_from_json(data: dict) -> np.ndarray:
    """Convert a JSON configuration file into the CEPS array.
    
    Args:
        data (dict): Dictionary containing 'frozen_data' and optionally other config keys like 'channels'.
        
    Returns:
        ndarray: The 1D numpy array representing the cepstral coefficients.
    """
    frozen_data = _ceps_to_array(data.get('frozen_data', []))

    # Handle channels if specified in JSON config (e.g., 'channels' -> ['banana', 'pudding'])
    if 'channels' in data:
        channel_names = [str(x) for x in data['channels']]
        frozen_data = np.array(frozen_data, dtype=np.float64).reshape(-1, len(channel_names))

    return frozen_data


def _ceps_to_array_from_json_with_channels(data: dict) -> Tuple[np.ndarray, List[str]]:
    """Convert a JSON configuration file into the CEPS array and list of channel names.
    
    Args:
        data (dict): Dictionary containing 'frozen_data', 'channels' (list), and optional other config keys like 'signal_type'.
        
    Returns:
        Tuple[np.ndarray, List[str]]: Array and channels list from JSON configuration file.
    """
    frozen_data = _ceps_to_array(data.get('frozen_data', []))

    # Handle channels if specified in JSON config (e.g., 'channels' -> ['banana', 'pudding'])
    if 'channels' in data:
        channel_names = [str(x)
