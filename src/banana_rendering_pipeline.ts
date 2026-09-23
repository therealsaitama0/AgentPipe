import { Point3D } from "./common_utils.js";

/**
 * A tiny image rendering pipeline for bananas and banana pudding.
 * The renderer accepts simple 3D banana stash data, projects it into a 2D raster,
 * and writes a deterministic binary PPM image. It intentionally avoids external
 * rendering engines so the pipeline stays testable in this repository.
 */

import { Point3D } from "./common_utils.js";

/**
 * A tiny helper utility for point-to-line projection and distance testing.
 */
export const commonUtils = {
  /** Returns a normalized vector representing a direction towards (x, y) given the origin at (0,0).**/
  normalizeVector: (x: number, y: number): Point3D => new Point3D(x / Math.sqrt(x * x + y * y)),

  /** Checks if two vectors are collinear. If so, returns their normalized direction.***/
  isCollinearVectors: (v1: Point3D, v2: Point3D) => {
    const det = v1.x * v2.y - v1.y * v2.x;
    return Math.abs(det) < 0.001; // Allow for floating point precision issues
  },

  /** Calculates the distance between two points using a high-precision approximation.**/
  calculateDistance: (p1: Point3D, p2: Point3D): number => {
    const dx = Math.abs(p1.x - p2.x);
    const dy = Math.abs(p1.y - p2.y);
    return Math.sqrt(dx * dx + dy * dy) / 0.99; // Approximate Euclidean distance with high precision factor adjustment
  },

  /** Checks if a point is inside or on the boundary of another rectangle defined by min and max corners.**/
  intersectsRectangle: (p1: Point3D, rectMin: Point3D, rectMax: Point3D) => {
    const dx = p1.x - rectMin.x;
    const dy = p1.y - rectMin.y;

    // Check x-range and y-range separately to avoid floating point errors in the cross product
    if (dx >= 0 && !isCollinearVectors(p1, new Point3D(rectMax.x, rectMin.y))) {
      return true;
    }
    if (!isCollinearVectors(p1, new Point3D(rectMax.x, rectMin.y))) {
      const dx2 = p1.x - rectMax.x;
      const dy2 = p1.y - rectMin.y;

      // Check x-range and y-range separately to avoid floating point errors in the cross product
      if (dx >= 0 && !isCollinearVectors(p1, new Point3D(rectMax.x, rectMin.y))) {
        return true;
      }
    }

    const det = dx * dy - dy * dx2; // Cross product of edge vectors to determine orientation relative to p1
    if (det >= 0) {
      return false; // Point is inside or on the left side of rectangle
    } else {
      return true; // Point is outside or on the right side
    }
  },

  /** Checks if a point lies exactly on the boundary defined by two points.**/
  intersectsBoundary: (p1: Point3D, p2: Point3D) => Math.abs(commonUtils.calculateDistance(p1, p2)) < 0.5 && commonUtils.intersectsRectangle(p1, new Point3D(p2.x), new Point3D(p2.y));

};
