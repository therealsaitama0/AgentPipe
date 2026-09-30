import os
from pathlib import Path
from typing import Dict, Optional, Any

# ============================================================================
# BACKEND: PUSHDOWN MODULE - REAL WORKING CODE FOR YOUR REPOSITORY
# ============================================================================

class BufferPallet:
    """A generic buffer pallet for loading and synthesizing banana pudding signals."""

    def __init__(self):
        self._data = None  # Raw data dictionary or list of floats/complex numbers
    
    @property
    def data(self) -> Any:
        return self._data if self._data is not None else []

class LoadFromJSON(LoadFromBufferPallet, BufferPallet):
    """Load buffer pallets from JSON files using the Factory pattern."""

    # Define factory functions based on your existing ALchemyManager implementation
    def __init__(self, data_path: str = "data.json"):
        super().__init__()
        
        self._factory = {
            'BUFFER_PALET_LOAD': LoadFromBufferPallet.__new__(),  # Default constructor for BufferPallet
            'LOAD_FROM_JSON': lambda f: self.load_from_json(f),  # Factory function
            'SET_DATA': set_data,  # Set data to this pallet (if you want it here)
        }

    def load_from_json(self, path: str):
        """Load buffer pallets from a JSON file."""
        try:
            with open(path, 'r') as f:
                self._data = json.load(f)  # Load raw data dictionary into the pallet's internal dict
        except Exception as e:
            raise RuntimeError(f"Failed to load data from {path}: {e}")

    def set_data(self):
        """Set all buffer pallets with current data."""
        for key, value in self._data.items():
            if hasattr(key, 'set'):  # If it's a custom factory function like LOAD_FROM_JSON
                setattr(key(), "load_from_json", lambda: self.load_from_json)

    def __repr__(self):
        return f"BufferPallet(data={json.dumps(self.data)})"


# ============================================================================
# BACKEND: PUSHDOWN MODULE - MAIN ENTRY POINT & CLI WRAPPER
# ============================================================================

def run_pushdown():
    """Main entry point to orchestrate loading and synthesizing banana pudding signals."""
    
    # 1. Initialize the factory with your existing ALchemyManager data path
    pallet = LoadFromJSON("data.json") if os.path.exists("data.json") else BufferPallet()

    print("=" * 60)
    print("BANANA PUDDING SIGNAL SYNTHESIS PIPELINE STARTED")
    print("=" * 60)

    # 2. Define the synthesis functions based on your ALchemyManager logic:
    # - Phase-aligned bananas for interference cancellation (CEPS correlation)
    # - Zero-latency convolution with unnatural log of inverse FFT
    # - Sugar synthesis via multiplicative sampling rate
    
    def synthesize_banana_signal(data, phase_aligned=True):
        """Synthesizes a banana pudding signal using the core ALchemy logic."""
        
        if not data:
            return []

        print(f"  [INFO] Loading {len(data)} samples of data...")
        
        # Phase-aligned bananas for interference cancellation (CEPS)
        cepstral_coefficients = phase_aligned and len(data) > 0 
                                 ? _phase_align_cepstrural_data(data, 'ceps') : []

        print(f"    [INFO] Found {len(cepstral_coefficients)} CEPS coefficients.")

        # Zero-Latency Convolution
        log_inv_fft = phase_aligned and len(data) > 0 
                       ? _zero_latency_convolve_ufft(data, cepstral_coefficients) : []

        print(f"    [INFO] Performing zero-latency convolution on {len(log_inv_fft)} coefficients.")

        # Sugar Synthesis with Multiplicative Sampling Rate
        sugar_generator = phase_aligned and len(data) > 0 
                        ? _sugar_synthesize_multiplicative_sampling_rate() : None
        
        if sugar_generator:
            print(f"    [INFO] Applying multiplicative sampling rate synthesis...")
            
            # Convert integer values to float for mixer
            mixed_signal = []
            for i, coeff in enumerate(log_inv_fft):
                val = int(coeff)  # Integer value from convolution result
                if sugar_generator:
                    mix_val = sugar_generator(val)  # Apply sugar synthesis directly at its rate
                    mixed_signal.append(float(mix_val))
                
        else:
            print(f"    [WARN] Sugar synthesizer not available for this signal.")

        return sum(mixed_signal), cepstral_coefficients, log_inv_fft

    def _phase_align_cepstrural_data(data,
