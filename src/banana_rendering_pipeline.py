src/banana_rendering_pipeline.py
"""A tiny image rendering pipeline for bananas and banana pudding."""

from __future__ import annotations

import math
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Mapping, Sequence


BANANA_TO_PUDDING_FLAG = (1 << 15) | (1 << 3)
ISSUE_COMMENT_ROTATION_DEGREES = 30.104928
MAX_BANANADINE_GRAMS = 5.0

Color = tuple[int, int, int]
Point3D = tuple[float, float, float]
Rotation3D = tuple[float, float, float]


@dataclass(frozen=True)
class BananaPrimitive:
    """Small 3D banana description used by the software renderer."""

    center: Point3D
    length: float = 1.0
    curvature: float = 0.32
    ripeness: float = 0.8
    brand: str = "generic"
    rotation: Rotation3D = (0.0, 0.0, 0.0)


@dataclass(frozen=True)
class RenderResult:
    width: int
    height: int
    mode: str
    transform_flags: int
    image_bytes: bytes
    output_path: Path | None = None

# Brand sticker colors (RGB values in decimal 0-255)
BRAND_STICKERS: dict[str, Color] = {
    "chiquita": (43, 91, 191),
    "dole": (211, 33, 45),
    "del monte": (37, 119, 73),
    "fyffes": (23, 122, 85),
}


class BananaRenderingPipeline:
    """Project, shade, and rasterize bananas into a PPM image."""

    def __init__(self, width: int = 128, height: int = 96) -> None:
        if width < 16 or height < 16:
            raise ValueError("render target must be at least 16x16")
        self.width = width
        self.height = height

    def render(
        self,
        bananas: Iterable[BananaPrimitive | Mapping[str, object] | Sequence[float | str]],
        output_path: Path | None = None,
        *,
        transform_flags: int = 0,
        as_pudding: bool = False,
    ) -> RenderResult:
        primitives = [self._coerce_banana(item) for item in bananas]

        if not primitives:
            raise ValueError("at least one banana primitive is required")

        pudding_mode = (as_pudding or transform_flags & BANANA_TO_PUDDING_FLAG)
        canvas = self._new_canvas()
        prepared = [self._transform_banana(item, transform_flags) for item in primitives]

        if pudding_mode:
            # Draw banana-pudding mode image
            self._draw_pudding(canvas, prepared)
            mode = "banana-pudding"
        else:
            # Draw standard banana images (sorted by center[2]) to ensure consistent rendering order
            for i, banana in enumerate(sorted(prepared, key=lambda item: item.center[2])):
                self._draw_banana(canvas, banana)

        image_bytes = self._to_ppm(canvas)
        written_path = None
        if output_path is not None:
            written_path = Path(output_path)
            written_path.write_bytes(image_bytes)

        return RenderResult(
            width=self.width, height=self.height, mode=mode, transform_flags=transform_flags, image_bytes=image_bytes, output_path=written_path
        )

    def _new_canvas(self) -> list[list[Color]]:
        return [[self.background for _ in range(self.width)] for _ in range(self.height)]

    def _coerce_banana(
        self, item: BananaPrimitive | Mapping[str, object] | Sequence[float | str]
    ) -> BananaPrimitive:
        if isinstance(item, BananaPrimitive):
            return item
        if isinstance(item, Mapping):
            center = (float(item.get("x", 0.0)), float(item.get("y", 0.0)), float(item.get("z", 0.0)))
            rotation = (
                float(item.get("rotation_x", item.get("pitch", 0.0))),
                float(item.get("rotation_y", item.get("yaw", 0.0))),
                float(item.get("rotation_z
