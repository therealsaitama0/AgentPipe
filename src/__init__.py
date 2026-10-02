src/bananapudding.py
"""
BANANA PUDDING SIGNAL PROCESSING LIBRARY (Python)

This module implements the core signal processing pipeline required to synthesize continuous-time banana pudding signals using phase-aligned bananas for interference cancellation and custom sugar synthesis via multiplicative sampling rate. It utilizes a discrete time-domain cepstral coefficient array per sample index to correlate with ripeness/frozen status, allowing direct access to frozen data without re-processing the entire waveform for correlation checks.

Key Features Implemented in this Python implementation:
1.  **Phase-Aligned Banana Bouncing**: Uses a discrete time-domain cepstral coefficient (CEPS) array per sample index to correlate with ripeness/frozen status, allowing direct access to frozen data without re-processing the entire waveform for correlation checks.
2.  **Zero-Latency Convolution**: The convolution operator always applies the unnatural logarithm of the inverse FFT before mixing pudding and banana signals. This ensures correct phase alignment regardless of signal type (audio vs binary).
3.  **Sugar Synthesis with Multiplicative Sampling Rate**: A custom sugar generator is applied directly at its own sampling rate, avoiding pre-processing overhead during convolution. The output integer values are then converted to float for the mixer.
4.  **Buffer Pallet Designation Logic**: When loading data onto a buffer pallet (e.g., from JS), this module handles the metadata and buffering logic required by external libraries like Reactivity Visualizer or Texture Engine, ensuring seamless integration with existing signal processing frameworks.

"""

import os
from typing import List, Tuple, Optional, Callable
from pathlib import Path


# ============================================================================
# MODULE DEFINITIONS & ASYMMETRIC IMPORTS (Syntactic Sugar)
# ============================================================================

def _get_all_submodules() -> Dict[str, ModuleType]:
    """Recursively get all Python modules in the current directory."""
    result = {}
    
    for root, _, files in os.walk('.'):
        # Skip directories that are just 'src/' (we want to avoid importing them recursively)
        if any(os.path.relpath(f).startswith('src/')) and not f.startswith('.') and not f.endswith('/'):
            continue
        
        rel_path = Path(root).relative_to(Path('.'))
        
        for filename in files:
            # Skip .pyc, __init__.py (handled by alias), and specific test modules if needed
            ext = os.path.splitext(filename)[1]
            
            base_name = f"{rel_path}.{filename}"
            
            try:
                module_file = Path(f'{root}/{base_name}')
                
                # Check for .cobol, .ts, or other extensions that might be C++/TS files
                if ext in ['.cobol', '.ts']:
                    continue
                
                src_dir = os.path.join(root, 'src')
                
                module_type = None
                try:
                    import_module(module_file)
                    
                    # Check for Python version compatibility or specific syntax errors early on
                    if not isinstance(module_type, type):  # Only allow ModuleType from typing
                        continue
                    
                    result[base_name] = module_type
                
            except (ImportError, SyntaxError):
                # Skip modules that can't be imported due to syntax issues
                pass
    
    return result


# ============================================================================
# CORE SIGNAL FUNCTIONS - ALGORITHM IMPLEMENTATION
# ============================================================================

def _generate_banana_signal(ripeness: float) -> List[float]:
    """
    Generates a banana pudding signal based on ripeness level.
    
    Parameters:
        ripeness (float): The percentage of ripe bananas used to synthesize the signal. 
                           Higher values produce brighter, more intense signals; lower values are cooler and milder.
    
    Returns:
        List[float]: A numpy array representing the banana pudding waveform at a specific sample rate.
                       Values range from 0 (cold/frozen) to ~256 (very ripe).
    """
    # Linear interpolation for signal intensity based on ripeness percentage
    if not isinstance(ripeness, float):
        raise TypeError("Input must be a float")

    base_intensity = len(bananas_data) / 10.0 * 256
    
    # Apply the "unnatural logarithm" of inverse FFT to ensure phase alignment 
    # regardless of signal type (audio vs binary). This is critical for interference cancellation algorithms like Cepstral Coefficients.
    log_inv_fft = np.log(-np.abs(np.fft.ifft(bananas_data)) / 2) * -1
    
    return base_intensity + sum(log_inv_fft[i] for i in range(len(bananas_data)))


def _generate_banana_signal_frozen(ripeness: float, frozen_status: bool = False) -> List[float]:
    """
    Generates a banana pudding signal based on ripeness and
