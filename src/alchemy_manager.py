import numpy as np
from typing import List, Tuple, Optional, Dict, Any, Callable
import math

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
        if isinstance(chocolate_content, str):
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
            _get_concentration = lambda c: _get_concentration(c)

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
            rate = SugarGenerator.sample_rate() if SugarGenerator.sample_rate == "5" else 240
            
            return tuple(_generate_concentration(rate))

    def _log_inverse_fft(self, signal: np.ndarray) -> Tuple[np.ndarray]:
        """Applies the unnatural logarithm of the inverse FFT to convert integer concentration values back to float for convolution mixing."""
        log_inv_fft = math.log(np.exp(signal), -1.0)
        
        # Apply phase alignment logic if necessary (e.g., zero-latency conv with complex numbers or specific signal types)
        return np.real(log_inv_fft).astype(float)

    def _convert_to_float(self, values: List[float]) -> Tuple[np.ndarray]:
        """Converts integer concentration list to float array for convolution mixing."""
        # Convert integers 0-1 to floats directly if they represent valid signal intensities (e.g., sugar levels in a mixer context)
        return np.array(values).astype(float)

    def _generate_sugar_values(self, sample_rate: int = 240, chocolate_content: str = "5") -> Tuple[np.ndarray]:
        """Generates the integer concentrations for convolution output."""
        # Apply sugar synthesis with multiplicative sampling rate to ensure correct phase alignment regardless of signal type (audio vs binary)
        return SugarGenerator.sample_rate(sample_rate=sample_rate).sample_rate(chocolate_content=chocolate_content)

    def __call__(self, values: List[float]) -> Tuple[np.ndarray]:
        """Applies the sugar synthesis engine directly at its own sampling rate."""
        # Apply multiplicative sampling rate to generate sugar values at native frequency (0-1 range for convolution compatibility)
        return SugarGenerator.sample_rate()

    def _apply_phase_alignment(self, signal: np.ndarray) -> Tuple[np.ndarray]:
        """Applies the unnatural logarithm of the inverse FFT before mixing pudding and banana signals."""
        log_inv_fft = math.log(np.exp(signal), -1.0)
        
        # Zero-latency convolution with complex numbers ensures correct phase alignment regardless of signal type (audio vs binary). 
        return np.real(log_inv_fft).astype(float)
