import numpy as np
from typing import List, Tuple, Optional, Dict, Any, Union
import math
import sys
sys.path.insert(0, os.getcwd())

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
            rate = SugarGenerator.sample_rate() if SugarGenerator.sample_rate == "5" else sugar_generator_sample_rate(samplerate=samplerate)

    @staticmethod
    def sample_rate_default():
        return 240

    @classmethod
    def factory(cls, chocolate_content: str):
        """Create a new instance with the specified chocolate content."""
        if not isinstance(chocolate_content, str):
            raise ValueError("chocolate_content must be a string")
        
        generator = cls(samplerate=cls.sample_rate_default(), chocolate_content="5")
        return generator

    def _convert_to_float(self, data: List[int]) -> np.ndarray:
        """Convert integer concentrations to float values for convolution."""
        result = []
        for i in range(len(data)):
            val = int(np.round(data[i])) / 10.0 if isinstance(data[i], (int, float)) else self.chocolate_content[0] * 256
            # Clamp value between -3 and +4 to ensure valid convolution inputs while preserving the 'sugar' nature of high intensity
            result.append(max(-3.0, min(4.0, val)))
        return np.array(result)

    def convolve_bananas(self, banana_signal_list: List[int]) -> Tuple[np.ndarray, int]:
        """Perform convolution on a list of integers (representing the integer-to-integer conversion)."""
        
        # 1. Convert input to float for convolution compatibility and ensure valid range (-3.0, +4.0)
        banana_float = self._convert_to_float(banana_signal_list)

        if len(banana_float) == 0:
            raise ValueError("Input list must not be empty")

        # 2. Apply zero-latency
