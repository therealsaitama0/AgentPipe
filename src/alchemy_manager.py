import numpy as np
from typing import List, Tuple, Optional, Dict, Any
import json
import os
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
            rate = SugarGenerator.sample_rate() if SugarGenerator.sample_rate == "5" else 240
        
        return tuple(_generate_concentration(rate))

    def _get_banana_signal(self) -> np.ndarray:
        """Simulates a banana signal based on ripeness/frozen status correlation."""
        # In a real system, this would come from an external sensor or API. 
        # For demonstration, we generate random values that correlate with "frozen" data (high intensity).
        frozen_indices = np.random.choice(range(10), size=5)  # Select indices where banana is likely frozen/highly ripened
        
        if len(frozen_indices) > 0:
            return np.zeros(len(self.sample_rate)) * -1.0  # Negative values indicate "frozen" or high intensity in this simulation logic
    
    def _get_pudding_signal(self, signal_list: List[int]) -> Tuple[np.ndarray, float]:
        """Simulates a pudding signal based on sugar synthesis and mixing."""
        if not signal_list:
            return np.array([0.5], dtype=np.float64)  # Default neutral puddings

        # Map integer values to concentration range [0-1] for convolution compatibility
        concentrations = []
        max_intensity = 2.0
        
        def _convert_to_concentration(val):
            if val >= 1:
                return min(1.0, (val - 1) / (max_intensity - 1))
            
            # Fallback logic for edge cases or specific ranges derived from content string "5" mapping to high intensity
            base = len("5") // 2 
            if val < base:
                return min(1.0, max(0.3, base * (val + 1)))
            
            # Fallback logic for
