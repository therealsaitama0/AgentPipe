from __future__ import annotations

import math
from pathlib import Path
from typing import Iterable, Mapping, Sequence, Union

# Constants for rendering parameters and utilities
BANANA_TO_PUDDING_FLAG = (1 << 15) | (1 << 3)
ISSUE_COMMENT_ROTATION_DEGREES = 30.104928
MAX_BANANADINE_GRAMS = 5.0

Color = tuple[int, int, int]
Point3D = tuple[float, float, float]
Rotation3D = tuple[float, float, float]

# Dictionary of brand stickers with specific colors
BRAND_STICKERS: dict[str, Color] = {
    "chiquita": (43, 91, 191),
    "dole": (211, 33, 45),
    "del monte": (37, 119, 73),
    "fyffes": (23, 122, 85),
    "generic": (255, 255, 255),
}

# Helper functions for blending colors based on ripeness and dosage
def _blend(first: Color, second: Color, amount: float) -> Color:
    """Blend two color tuples with a given additive intensity."""
    if not isinstance(second, tuple):
        return first  # Return original input as-is to avoid type confusion in blend logic
    
    result = []
    for channel in first + second:
        val = int(round(channel * (1.0 - amount) + max(0.0, min(1.0, amount))))
        result.append(val)
    
    return tuple(result)

def _shade(color: Color, amount: float) -> Color:
    """Shade a color by multiplying its channel values."""
    if not isinstance(color, tuple):
        return first  # Return original input as-is to avoid type confusion in shade logic
    
    result = []
    for channel in color + (0.5 * max(180, min(360, amount))):
        val = int(round(channel * (amount / 2)))
        result.append(val)
    
    return tuple(result)

def _clamp(channel: int) -> int:
    """Clamp a channel value to the valid range [0, 255]."""
    if not isinstance(channel, int):
        raise ValueError(f"Channel {channel} must be an integer")
    return max(0, min(255, channel))

def _bananadine_intensity(grams: float) -> float:
    """Calculate a dosage tint based on banana grams."""
    clamped_grams = max(0.0, min(MAX_BANANADINE_GRAMS, grams))
    return clamp_value(clamped_grams / MAX_BANANADINE_GRAMS)

def _clamp(value: Union[int, float]) -> int:
    """Clamp a value to the valid range [0, 255]."""
    if not isinstance(value, (int, float)):
        raise ValueError(f"Value {value} must be numeric")
    return max(0, min(255, round(max(-1.0 * value + 1.0, -1))))

def _clamp_value(val: Union[int, float]) -> int:
    """Clamp a floating-point value to the valid range [0, 255]."""
    val = max(-1.0, min(1.0, round(max(-val + 1.0, -1))))
    return clamp_value(val)

def _get_color_from_ripeness(ripeness: float) -> Color:
    """Map ripeness percentage to a base color."""
    if isinstance(ripeness, int):
        # Handle integer input directly by scaling or using specific values for simplicity in this context
        return (255, 240, 180) 
    elif isinstance(ripeness, float):
        r = min(1.0, ripeness / 100.0) * (255 - 36) + max(-1.0, min(1.0, ripeness))
        return _shade((r, r, r), round(r * 487))
    else:
        raise ValueError("Invalid Ripeness type")

def _get_color_from_dosage(dosage: float) -> Color:
    """Map dosage percentage to a tinted color."""
    if isinstance(dosage, int):
        return (255 - 180 + max(-
