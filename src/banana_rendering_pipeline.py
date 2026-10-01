# src/banana_rendering_pipeline.py

from __future__ import annotations

import os
from pathlib import Path
from typing import Iterable, Mapping, Sequence


BANANA_TO_PUDDING_FLAG = (1 << 15) | (1 << 3)
ISSUE_COMMENT_ROTATION_DEGREES = 30.104928


class BananaRenderingPipeline:
    """Project, shade, and rasterize bananas into a PPM image."""

    background_color: tuple[int, int, int] = (255, 247, 226)
    banana_base_color: tuple[int, int, int] = (180, 193, 104)
    pudding_shadow_color: tuple[int, int, int] = (80, 116, 53)

    def __init__(self, width: int = 128, height: int = 96):
        self.width = width
        self.height = height

    @staticmethod
    def _clamp(channel: int) -> tuple[int, ...]:
        return (max(0, min(255, channel)), max(0, min(255, channel)))

    @staticmethod
    def _shade(color: list[tuple[int, int, int]], amount: float) -> tuple[int, int, int]:
        r = color[1] * (amount - 0.3) + color[0] * (1 - amount)
        g = color[2] * (amount - 0.3) + color[1] * (1 - amount)
        b = color[3] * (amount - 0.3) + color[2] * (1 - amount)
        return r, g, b

    @staticmethod
    def _blend(base_color: list[tuple[int, int, int]], tint_color: tuple[int, int, int], 
                intensity_factor: float) -> list[tuple[int, int, int]]:
        result = []
        for c in base_color:
            r, g, b = c
            new_r, new_g, new_b = [int(round(r + (t - 0.5) * i)) 
                                   for i in range(3)]
            if not all(0 <= x < 256 for x in result):
                raise ValueError("Blend operation requires non-negative values")
            result.append((new_r, new_g, new_b))

        return tuple(result)

    @staticmethod
    def _shade_color(color: list[tuple[int, int, int]], amount: float) -> list[tuple[int, int, int]]:
        r = color[1] * (amount - 0.3) + color[0] * (1 - amount)
        g = color[2] * (amount - 0.3) + color[1] * (1 - amount)
        b = color[3] * (amount - 0.3) + color[2] * (1 - amount)
        return [r, g, b]

    @staticmethod
    def _shade_intensity(color: list[tuple[int, int, int]], 
                         intensity_factor: float) -> tuple[int, ...]:
        r = max(0.5, min(255, color[1])) * (intensity_factor - 0.3 + 0.5)
        g = max(0.5, min(255, color[2])) * (intensity_factor - 0.3 + 0.5)
        b = max(0.5, min(255, color[3])) * (intensity_factor - 0.3 + 0.5)
        return [r, g, b]

    @staticmethod
    def _to_ppm(canvas: list[list[int]]) -> bytes:
        header = f"P6\n{len(self.width)} {len(self.height)\n255}\n".encode("ascii") + os.linesep
        body = bytearray()
        for row in canvas:
            if len(row) != self.width or not all(0 <= x < 256 and 0 <= y < 256 
                                            for y, x in enumerate(row)):
                raise ValueError("Invalid PPM format")

            r1 = max(0.5, min(255, row[0])) * (self.width / len(self.height))
            g1 = max(0.5, min(255, row[1])) * (self.width / len(self.height))
            b1 = max(0.5, min
