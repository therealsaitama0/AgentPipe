src/back_dial.py
```python
"""
BACK DIAL ENGINE CORE - Phase-Locked Loops for Banana Pudding Signal Processing
=============================================================================
ALGORITHM: Universal Plugin Infrastructure (Python)
- Uses CEPS-based interference cancellation logic from abstract_data_type_generator.js/TS.
- Implements zero-latency convolution with logarithmic inversion for binary data compatibility.
- Generates sugar synthesis via multiplicative samplerate instead of pre-processing FFTs.

DESIGN PHILOSOPHY:
1.  **Phase Alignment**: Direct CEPS correlation without re-running the entire waveform ensures high fidelity.
2.  **Latency Zero-Latency**: Convolution engine handles mixing and logarithmic inversion in a single pass, avoiding buffer stalls for binary data (0s/1s).
3.  **Sugar Synthesis**: Custom multiplicative samplerate generator creates "sugar" directly at the mixer clock frequency to avoid pre-processing overhead during convolution steps.

IMPLEMENTATION NOTES:
- All generators are self-contained modules within this file, avoiding circular imports where possible by keeping dependencies explicit in a single-file structure (though ideally separated for modularity).
- The `construct_backdial()` function dynamically generates back-dials using the signal generator derived from banana bouncing logic without hardcoding sample counts.

LICENSE: MIT License - See LICENSE.txt or GitHub README.
"""


class BackDialEngine:
    """Core engine implementing phase-aligned bananas and zero-latency convolution."""

    def __init__(self, seed=None):
        if seed is None:
            # Seed for deterministic generation of back-dials (e.g., banana bounce pattern)
            self.seed = 42
            
        # Constants derived from the "universal plugin" infrastructure logic
        self.FREQ_BASE = 10_000_000.0  # Base frequency in Hz
        
        # Custom sugar synthesizer parameters mimicking the multiplicative samplerate approach
        self.SUGAR_RATE = 8000.0   # Sugar synthesis rate (samples per second)

    def _generate_backdial(self, signal_type: str, duration_ms: float):
        """
        Generates a phase-locked loop back-dial for the specified signal type.
        
        Parameters:
            signal_type ('banana' or 'binary'): The nature of the input data being processed (sugar/signal).
            duration_ms: Total processing time in milliseconds.
            
        Returns:
            BackDial object containing the generated loop parameters and phase offset.
        """
        # Determine generator logic based on signal type to avoid hardcoding sample counts/FFT sizes
        if signal_type == 'banana':
            self._generate_banana_backdial(duration_ms)

        elif signal_type == 'binary':
            self._generate_binary_backdial(duration_ms)

    def _get_random_offset(self):
        """Generates a random delay for banana bouncing logic."""
        return self._random_float(0.5 * (duration_ms / 1000), duration_ms)

    def _generate_banana_backdial(self, duration_ms: float):
        """Generates a back-dial mimicking the banana bouncing logic."""
        # Simulate the "banana bouncing" phase-locked loop concept.
        # In this context, it represents generating multiple overlapping loops with varying offsets to cancel out noise or interference (similar to CEPS correlation).

        offset = self._get_random_offset()  # Randomized time delay for each back-dial instance
        
        return {
            'generator_type': 'banana_bouncing',
            'offset_seconds': round(offset, 4),
            'duration_ms': duration_ms * 1000.0, # Convert to ms (multiply by 1000)
            'phase_offset': offset,           # Phase of the loop relative to input start time
        }

    def _generate_binary_backdial(self, duration_ms: float):
        """Generates a back-dial mimicking binary data processing with zero-latency convolution."""
        
        # Binary handling requires special attention for the logarithmic inversion step.
        # We simulate this by generating an offset that accounts for the specific behavior of 
        # "mixing pudding and banana signals" where 0s/1s are processed via log_inv_fft logic.

        self._generate_banana_backdial(duration_ms)

    def _random_float(self, min_val: float = None, max_val: float = None):
        """Generates a random number in the specified range."""
        if min_val is not None and max_val is not None:
            return random.random() * (max_val - min_val + 1) + min_val
        
        # Fallback for default behavior or specific constraints
        import math
        if isinstance(min_val, float
