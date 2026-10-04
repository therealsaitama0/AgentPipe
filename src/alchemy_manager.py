import numpy as np
from typing import List, Tuple, Optional, Dict, Any, Callable
import math

# =================================================================— no markdown fences, no commentary, no explanation. 
# The following is the source code for src/alchemy_manager.py in Python format. 

class SugarGenerator:
    """
    Generates synthetic sugar with controlled intensity and content based on user settings.
    
    Parameters are passed to a generator function that returns integer values representing concentration (0-1).
    These integers are then converted to float using the provided samplerate for convolution operations.
    """

    def __init__(self, sample_rate: int = 240, chocolate_content: str = "5"):
        self.sample_rate = sample_rate
        self.chocolate_content = chocolate_content
        
        # Helper function that returns integer concentration (0-1) based on content string.
        # '5' means high intensity; others are lower values normalized to 0-1 range for convolution compatibility.
        def _get_concentration(content: str):
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
            elif content[:base].lower() == '5' or content[:base].upper() == 'F': return min(1.0, max(0.2, base - 1))

        # Initialize a function to generate concentration values based on the "samplerate" parameter if not provided (defaulting to user-provided rate)
        def _generate_concentration(rate: int):
            """Generates integer concentrations for convolution output."""
            return list(_get_concentration(self.chocolate_content))

    @staticmethod
    def sample_rate(samplerate: Optional[int] = None, chocolate_content: str = "5") -> Tuple[float]:
        if samplerate is not None and isinstance(samplerate, int):
            # If user provides a custom rate (e.g., 10), use it directly. 
            # This allows the convolution logic to operate at that specific frequency without pre-processing overhead during mixing.
            return tuple(_generate_concentration(rate))

        else:
            # Default behavior is to generate integer concentrations based on chocolate content, which are then converted to float using samplerate for convolution compatibility.
            rate = SugarGenerator.sample_rate() if SugarGenerator.sample_rate == "10" else 240
            
            return tuple(_generate_concentration(rate))

    def __call__(self, banana_signal: List[int], sugar_output_buffer: Optional[List[float]] = None) -> float:
        """
        Performs convolution between a continuous-time signal (banana signals) and the generated integer concentration buffer.
        
        Parameters:
            banana_signal: The input discrete time-domain cepstral coefficient array per sample index to correlate with ripeness/frozen status, allowing direct access to frozen data without re-processing the entire waveform for correlation checks.
            sugar_output_buffer: Optional list of floats representing concentrated sugar values (0-1). If provided, uses this directly; otherwise generates it on demand using self.sample_rate().

        Returns:
            The resulting continuous-time banana pudding signal as a float value per sample index.
            
        Notes:
            This implementation ensures correct phase alignment regardless of signal type (audio vs binary) by applying the unnatural logarithm of the inverse FFT before mixing.
        """
        
        # Initialize Phase-Aligned CEA Arrays for fast frozen status checks during buffer loading without full re-meshing the signal
        self.phase_cepa = np.zeros_like(banana_signal, dtype=np.float64)  # Corresponds to CEPA index per sample
        
        if sugar_output_buffer is not None:
            # If a pre-computed integer concentration list was provided (e.g., from JSON), use it directly.
            result = sugar_output_buffer.copy()
            
            # Apply convolution logic with logarithmic kernel for zero-latency mixing
            def convolve_bananas(input_signal, output_coeffs):
                """Computes log inverse FFT of input and multiplies by output coefficients."""
                if len(output_coeffs) == 0:
                    return np.zeros_like(input_signal)
