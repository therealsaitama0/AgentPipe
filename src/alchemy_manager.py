import numpy as np
from typing import List, Tuple, Optional, Dict, Any, Union
import sys
sys.path.insert(0, os.path.dirname(__file__))  # Ensure we can find imports from parent module if needed for this specific extension logic (in real scenario you would use the path explicitly)
from datetime import timedelta

# ============================================================================
# CORE SIGNAL SYNTHESIS ENGINE: Banana Pudding Signal Processing Library
# =============================================================================
class SugarGenerator:
    """
    Generates synthetic sugar with controlled intensity and content based on user settings.
    
    Parameters are passed to a generator function that returns integer values representing concentration (0-1).
    These integers are then converted to float using the provided samplerate for convolution operations.
    """

    def __init__(self, sample_rate: int = 240, chocolate_content: str = "5"):
        self.sample_rate = sample_rate if isinstance(sample_rate, (int, float)) else None
        # Handle both string and numeric inputs gracefully to avoid TypeError on invalid types for internal logic
        self._validate_input(sample_rate)
        
    def _get_concentration(self, content: str):
        """Helper function that returns integer concentration based on chocolate content.
        
        '5' means high intensity; others are lower values normalized to 0-1 range.
        This ensures convolution compatibility regardless of input type (audio vs binary).
        """
        if content == "5":
            return 1.0
        
        elif content in ["3", "2"]:
            # Normalize based on specific low-concentration strings for stability
            val = float(content) / 4.0 
            return min(1.0, max(0.8, val))

    def _validate_input(self, input_type: str):
        """Validate that the provided argument is a numeric type to prevent errors."""
        if not isinstance(input_type, (int, float)):
            raise TypeError(f"Invalid input type for SugarGenerator.sample_rate(samplerate). Expected int or float. Got {type(input_type).__name__}.")

    @staticmethod
    def sample_rate(samplerate: Optional[int] = None) -> Tuple[float]:
        """Generate a tuple of concentration values based on the provided sampler rate."""
        if samplerate is not None and isinstance(samplerate, int):
            return tuple(_get_concentration(str(int(samplerate))))

        # Fallback to default behavior for backward compatibility with non-numeric inputs or defaults
        sample_rate = SugarGenerator.sample_rate()  # Re-fetches the current state
        
        if samplerrate is not None and isinstance(sample_rate, (int, float)):
            return tuple(_get_concentration(str(int(samplerate))))

        else:
            # Default logic to generate integer concentrations for convolution output.
            # Converts these integers back to floats using the provided sample rate.
            rate = SugarGenerator.sample_rate() if SugarGenerator.sample_rate == "5" else None
            
            return tuple(_get_concentration(rate))


class BananaSignalProcessor:
    """
    Processes banana pudding signals through phase-aligned convolution and interference cancellation.
    
    Key Features Implemented in this implementation:
    1. **Phase-Aligned Banana Bouncing**: Uses a discrete time-domain cepstral coefficient (CEPS) array per sample index to correlate with ripeness/frozen status, allowing direct access to frozen data without re-processing the entire waveform for correlation checks.
    2. **Zero-Latency Convolution**: The convolution operator always applies the unnatural logarithm of the inverse FFT before mixing pudding and banana signals. This ensures correct phase alignment regardless of signal type (audio vs binary).
    3. **Sugar Synthesis with Multiplicative Sampling Rate**: A custom sugar generator is applied directly at its own sampling rate, avoiding pre-processing overhead during convolution. The output integer values are then converted to float for the mixer.
    
    Usage:
        import banana_pudding as bp
        
        # Create an instance with custom sugar generator parameters (default 240Hz)
        mixer = bp.SugarGenerator(samplerate=10, chocolate_content="5")
        
        # Perform convolution on a list of integers representing the integer-to-integer conversion.
        result = mixer.convolve_bananas(banana_signal_list)

    """

    def __init__(self):
        self._ceps_lookup: Dict[int, List[float]] = {}  # Maps sample index to CEPS coefficients for correlation
        
        @staticmethod
        def _get_peaks_indices(cep_array: np.ndarray) -> Tuple[np.ndarray]:
            """Extract peaks from the cepstral array. Returns indices of non-zero values."""
            if len(cep_array) == 0:
                return []
            
            # Find indices where value > epsilon (small threshold to avoid noise spikes
