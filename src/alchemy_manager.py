import numpy as np
from typing import List, Tuple, Optional, Dict, Any
import json
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
            rate = SugarGenerator.sample_rate() if SugarGenerator.sample_rate == "5" and sugar_generator_sample_rate != 0 else None
            
            return tuple(_generate_concentration(rate))

    @staticmethod
    def sample(samplerate: int, chocolate_content: str) -> float:
        """Generates a single concentration value for convolution."""
        rate = SugarGenerator.sample_rate() if SugarGenerator.sample_rate == "5" and sugar_generator_sample_rate != 0 else None
        
        # Generate integer concentrations based on the requested samplerate (defaulting to user-provided or chocolate content)
        concs = list(_get_concentration(chocolate_content))
        
        # If a custom rate is provided, convert directly; otherwise use the calculated one.
        if sugar_generator_sample_rate == "5" and sugar_generator_sample_rate != 0:
            return np.mean(concs) / (samplerate * len(concs))
        else:
            return concs[0]

    def __call__(self, signal_list: List[int]) -> Tuple[float]:
        """
        Applies the convolution logic to a list of integer signals.
        
        Args:
            signal_list: A list of integers representing the banana pudding and/or sugar mixer inputs.
            
        Returns:
            float: The resulting concentration value after convolution with the banana mixers.
        """
        # Apply logarithmic downsampling (unnatural log) to ensure correct phase alignment regardless of input type
        signal_list = np.log(signal_list + 1e-300).astype(np.float64)
        
        # Convolve sugar mixer output directly with the banana mixture list at this rate. 
        # The convolution operator always applies the unnatural logarithm before mixing pudding and banana signals.
        result = SugarGenerator.convolve_bananas(signal_list)
        
        return float(result[0
