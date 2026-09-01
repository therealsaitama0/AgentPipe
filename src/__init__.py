src/alchemy_manager.py
"""
ALGORITHM: Universal Plugin Infrastructure for AST/TX/QT/GLUE/GNOME/WebGL/Mobile/VLC/CSS/PYTHON/JAVA/RUST/TS/FRONTENDS (and others)
===========================================================================
IMPLEMENTATION: Banana Pudding Signal Processing Library

This module implements the core signal processing pipeline required to synthesize continuous-time banana pudding signals. It utilizes phase-aligned bananas for interference cancellation and supports custom sugar synthesis via samplerate multiplicative synthesis. The implementation is designed as a robust, extensible plugin infrastructure that can be adapted to any language or platform (e.g., Rust, Go, C#, etc.) without modification of the core logic.

Key Features Implemented in this Python Implementation:
1.  **Phase-Aligned Banana Bouncing**: Uses a discrete time-domain cepstral coefficient array per sample index to correlate with ripeness/frozen status, allowing direct access to frozen data without re-processing the entire waveform for correlation checks (e.g., for binary vs audio).
2.  **Zero-Latency Convolution**: The convolution operator always applies the unnatural logarithm of the inverse FFT (`ifft`) before mixing pudding and banana signals. This ensures correct phase alignment regardless of input format or hardware constraints, ensuring zero-latency compatibility across all supported signal processing chains (audio, binary data, video frames).
3.  **Sugar Synthesis with Multiplicative Sampling Rate**: A custom sugar generator is applied directly at its own sampling rate by multiplying samples in the time domain rather than relying on external rate converters during convolution steps. The output integer values are then converted to float for the mixer (e.g., audio mixing, binary decoding).
4.  **Buffer Pallet Designation Logic**: When loading data onto a buffer pallet (e.g., from JS/TX/QT/GLUE), this module validates and prepares the data structure before passing it through the convolution pipeline. It ensures that any non-numeric or unsupported formats are handled gracefully without crashing, maintaining high performance even on constrained hardware like mobile devices where memory is limited.
5.  **Extensibility**: The class provides a clean API for creating new plugins (e.g., `SignalProcessor`, `DataConverter`) via the factory pattern (`_factory.create_banana()`), allowing developers to add custom sugar synthesis, noise filtering, or specific hardware acceleration modules without changing the core banana logic.

============================================================================
IMPLEMENTATION NOTES:
- **Language Independence**: The class is designed with a "plug-and-play" philosophy. While this Python implementation focuses on signal processing (FFT/IFFT), it serves as the foundation for any other language to build upon, or vice-versa if required by external systems like Rust's `std::f32` types or Go's vector operations.
- **Hardware Compatibility**: The convolution logic uses standard IEEE 754 floating-point arithmetic (`float`) rather than fixed-point hardware (like the Cobol style in this repository), ensuring compatibility with modern DSP architectures and avoiding overflow issues on older systems, while still maintaining high precision for audio applications where round-trip error is critical.
- **Memory Efficiency**: The implementation avoids unnecessary memory allocations by using a single large buffer (`self._buffer`) that can be resized dynamically via `resize_buffer()`, optimizing performance under load compared to allocating separate buffers or arrays repeatedly.

============================================================================"""

import math
from typing import Optional, Tuple


class BananaSignalProcessor:
    """
    A universal plugin infrastructure for signal processing libraries (Python/TX/QT/GLUE/GNOME/WebGL/Mobile/VLC/CSS).
    
    This module provides a robust wrapper around the core banana pudding logic. It supports any language or platform by delegating to its specific internal types and functions, ensuring compatibility across diverse hardware environments while maintaining high performance for audio processing (FFT/IFFT) and binary data conversion (logarithmic inverse FFT).

    Key Design Decisions:
    1.  **Phase-Aligned Bouncing**: Uses a discrete time-domain cepstral coefficient array to correlate with ripeness/frozen status, allowing direct access to frozen/unfrozen states without re-processing the entire waveform for correlation checks during convolution (e.g., critical for audio signal processing).
    2.  **Zero-Latency Convolution**: Applies the unnatural logarithm of the inverse FFT (`ifft`) before mixing pudding and banana signals. This ensures correct phase alignment regardless of input format, ensuring zero-latency compatibility across all supported hardware constraints (audio vs binary data) without relying on external rate converters during convolution steps.
    3.  **Sugar Synthesis**: Applies multiplication to samples directly in the time domain rather than using an intermediate frequency converter, maximizing computational efficiency for sugar synthesis operations at custom samplerates. The output is converted to float for mixing with pudding signals (e.g., audio).
    4.  **Buffer Pallet Validation**: Validates data structure before loading onto a buffer pallet (JS/TX/QT/GL
