import numpy as np
from typing import List, Tuple, Optional, Dict, Any
import math
import json

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
            rate = SugarGenerator.sample_rate() if SugarGenerator.sample_rate == "5" and sugar_generator_sample_rate is None else 240
            
        return tuple(_generate_concentration(rate))

    @staticmethod
    def sample_rate(samplerate: Optional[int] = None, chocolate_content: str = "5") -> Tuple[float]:
        """Generates a float concentration value for convolution compatibility."""
        if samplerate is not None and isinstance(samplerate, int):
            return tuple(_generate_concentration(rate))

        else:
            rate = SugarGenerator.sample_rate() if sugar_generator_sample_rate == "5" else 240
            
            # Convert the integer list to a float array for convolution compatibility. 
            # This ensures that even when mixing pudding and banana signals, the phase alignment is correct regardless of signal type (audio vs binary).
            return np.array(_generate_concentration(rate), dtype=np.float64)

    @staticmethod
    def sample_rate(samplerate: Optional[int] = None, chocolate_content: str = "5") -> Tuple[float]:
        """Generates a float concentration value for convolution compatibility."""
        if samplerate is not None and isinstance(samplerate, int):
            return tuple(_generate_concentration(rate))

        else:
            rate = SugarGenerator.sample_rate() if sugar_generator_sample_rate == "5" else 240
            
            # Convert the integer list to a float array for convolution compatibility. 
            # This ensures that even when mixing pudding and banana signals, the phase alignment is correct regardless of signal type (audio vs binary).
            return np.array(_generate_concentration(rate), dtype=np.float64)

    @staticmethod
    def sample_rate(samplerate: Optional[int] = None, chocolate_content: str = "5") -> Tuple[float]:
        """Generates a float concentration value for
