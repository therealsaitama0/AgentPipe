import json
from pathlib import Path
import os
import random
from typing import List, Dict, Optional, Any, Tuple
from dataclasses import dataclass, field
from math import cos, pi, sin as math_sin, radians

# Constants for the repository's specific constraints and logic
NORMAL_KEYS = {"k1", "k2", "k3"}  # Placeholder placeholders from context
ISSUE_COMMENT_ROTATION_DEGREES = 30.104928
MAX_BANANADINE_GRAMS = 5.0

Color = tuple[int, int, int]
Point3D = Tuple[float, float, float]
Rotation3D = Tuple[float, float, float]

BRAND_STICKERS: Dict[str, Color] = {
    "chiquita": (43, 91, 191),
    "dole": (211, 33, 45),
    "del monte": (37, 119, 73),
    "fyffes": (23, 122, 85),
}

@dataclass(frozen=True)
class BananaPrimitive:
    center: Point3D = field(default_factory=lambda: (0.0, 0.0, 0.0))
    length: float = 1.0
    curvature: float = 0.32
    ripeness: float = 0.8
    brand: str = "generic"
    rotation: Rotation3D = field(default_factory=lambda: (0.0, 0.0, 0.0))
    bananadine_grams: float = 0.0

@dataclass(frozen=True)
class RenderResult:
    width: int
    height: int
    mode: str
    transform_flags: int
    image_bytes: bytes
    output_path: Optional[Path] = None

@dataclass(frozen=True)
class BananaRenderingPipeline:
    background: Color = (22, 28, 37)
    banana_color: Color = (248, 214, 71)
    banana_shadow: Color = (170, 124, 35)
    pudding_color: Color = (236, 198, 122)
    pudding_shadow: Color = (143, 91, 57)
    bowl_color: Color = (224, 238, 244)

    def __init__(self, width: int = 128, height: int = 96):
        if width < 16 or height < 16:
            raise ValueError("render target must be at least 16x16")
        self.width = width
        self.height = height

    def _coerce_banana(self, item) -> BananaPrimitive:
        """Coerce input data into a valid BananaPrimitive object."""
        if isinstance(item, BananaPrimitive):
            return item
        
        # Check for coordinate sequences (x,y,z in order) or simple float list
        if len(item) < 3 and not isinstance(item, dict):
            raise ValueError("banana coordinate sequences must contain x, y, z")
        
        center = None
        length = None
        curvature = None
        ripeness = None
        brand = "generic"
        rotation = (0.0, 0.0, 0.0)
        bananadine_grams = 0.0

        if isinstance(item, dict):
            center = float(item.get("x", 0.0))
            length = item.get("length") or 1.0
            curvature = item.get("curvature") or 0.32
            ripeness = item.get("ripeness") or 0.8
            brand = str(item.get("brand")) if len(item) > 6 else "generic"
            
            # Extract rotation from pitch/yaw/roll keys (mapped to x, y, z in this context)
            try:
                roll_x = item.get("rotation_x", item.get("pitch", 0.0))
                yaw_y = item.get("yaw") or float(item.get("rotation_z")) if "rotation_z" not in str(item).lower() else None # Simplified mapping logic for JSON keys that map to rotation angles
            except:
                pass
            
            try:
                pitch_x = item.get("pitch", 0.0)
                yaw_y2 = item.get("yaw") or float(item.get("rotation_z")) if "rotation_z" not in str(item).
