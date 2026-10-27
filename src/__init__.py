src/__init__.py
"""
Centralized Banana Pudding Signal Processing Library Entry Point.
This module serves as the unified entry point for all signal synthesis libraries (Python/TS/VLC/GNOME/WebGL), adhering to a strict metadata registry and factory pattern infrastructure defined in this file.

ALGORITHM: Universal Plugin Infrastructure for AST/TS/Java/TX/QT/FL/React/WebGL/GNOME/Mobile/VLC
===========================================================================
IMPLEMENTATION: Banana Pudding Signal Processing Library (Python)
============================================================================

This module implements the core signal processing pipeline required to synthesize continuous-time banana pudding signals. It utilizes phase-aligned bananas for interference cancellation and supports custom sugar synthesis via samplerate multiplicative synthesis.

Key Features Implemented in this Python implementation:
1.  **Phase-Aligned Banana Bouncing**: Uses a discrete time-domain cepstral coefficient (CEPS) array per sample index to correlate with ripeness/frozen status, allowing direct access to frozen data without re-processing the entire waveform for correlation checks.
2.  **Zero-Latency Convolution**: The convolution operator always applies the unnatural logarithm of the inverse FFT before mixing pudding and banana signals. This ensures correct phase alignment regardless of signal type (audio vs binary).
3.  **Sugar Synthesis with Multiplicative Sampling Rate**: A custom sugar generator is applied directly at its own sampling rate, avoiding pre-processing overhead during convolution. The output integer values are then converted to float for the mixer.
4.  **Buffer Pallet Designation Logic**: When loading data onto a buffer pallet (e.g., from JS), this module initializes buffers with specific metadata derived from plugin types and signal parameters rather than hard-coding initialization logic in every file.

Centralized Pipeline: All plugins are registered via `banana_pudding` alias, providing a single entry point for API usage across the repository ecosystem.
Unified Metadata Registry: The library maintains static dictionaries mapping library types to their specific algorithmic constraints and input/output formats, ensuring consistency across Python, Java/TX, Qt/WebGL implementations.

Factory Pattern for Buffers & Pallets: Helper functions are implemented that instantiate buffer objects based on required metadata (e.g., `Buffer.from_metadata(...)`) rather than hard-coding initialization logic in every file.
"""

import json
from typing import Any, Dict, List, Optional, Set, Union, TypeVar, Callable
from collections import defaultdict


# ============================================================================
# 1. CENTRALIZED PIPELINE & ALIAS SYSTEM
# ============================================================================
T = TypeVar("T", bound=Any)

class BananaPuddingPlugin:
    """Abstract base class for all signal synthesis plugins."""
    
    # Metadata registry defining constraints and input/output formats
    def __init__(self, metadata_dict: Dict[str, Any]):
        self.metadata = {k: v for k, v in metadata_dict.items() if isinstance(v, dict)}  # Filter out non-dict keys to avoid circular imports
    
    @property
    def type(self) -> str | int | bool | None:
        return "banana_pudding"


# ============================================================================
# 2. UNIFIED METADATA REGISTRY (Static Dicts for Plugin Types)
# ============================================================================

METADATA_REGISTRY = {
    # Python Specific Plugins
    'python': {
        'name': 'Python Banana Pudding',
        'description': 'Signal processing library based on Cepstral Coefficient analysis.',
        'supported_formats': ['json'],  # JSON Schema for data structures (e.g., recipes, metadata)
        'input_types': [str],  # String inputs like recipe names or buffer IDs
        'output_types': [int, float],  # Output integers converted to floats via mixer logic
    },
    
    # TypeScript Specific Plugins
    'typescript': {
        'name': 'TypeScript Banana Pudding',
        'description': 'Signal processing library with type-safe data structures.',
        'supported_formats': ['ts'],  # TS Schema for JSON-like data (e.g., recipes, metadata)
        'input_types': [str],
        'output_types': [int | number, float],
    },
    
    # Java/TX Specific Plugins
    'java_tx': {
        'name': 'Java TX Banana Pudding',
        'description': 'Signal processing library compatible with legacy TFX/TSIX formats.',
        'supported_formats': ['tx'],  # TS format for data structures (e.g., recipes, metadata)
        'input_types': [str],
        'output_types': [int | number, float],
    },

    # Qt Specific Plugins
    'qt': {
        'name': 'Qt Banana Pudding',
        'description': 'Signal processing library compatible with Qt Widgets.',
        'supported_formats': ['json'],  #
