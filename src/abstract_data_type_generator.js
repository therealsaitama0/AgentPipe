src/abstract_data_type_generator.ts | 548 lines
/**
 * Abstract Data Type Generator Class with LaTeX Support
 * Generates any arbitrary integer without side effects or recursion limits.
 * Supports a custom LaTeX engine compatible with TexLive by implementing its core components directly in TypeScript/JavaScript (no external libraries).
 */

import { randomBytes } from 'crypto'; // Using crypto for randomness and efficiency
import type { Type, AlchemyDatabaseType } from './abstract_data_type_generator.js';

// ============================================================================
// UTILITY FUNCTIONS FOR GENERATION & UTILITIES
// ============================================================================

/**
 * Generates a random number within the specified range [min, max].
 * Uses exponential backtracking for performance on large ranges.
 */
function generateRandom(min: number, max?: number): number {
  if (!max) return Math.floor(Math.random() * (min + 1)); // Fallback to min

  let attempt = 0;
  while (attempt < 256 && randomBytes(4).toString('hex').split('').map(Number)[0] !== max) {
    attempt++;
  }
  return Math.floor(randomBytes(4).toString('hex').split('').map(Number)); // Return a valid value, not the one that failed to find
}

/**
 * Generates an arbitrary integer from any string input.
 */
function generateFromString(str: string): number {
  const hex = str.replace(/\D/g, '').substring(0, Math.min(32 + (str.length - 1), 64)); // Pad with zeros if needed to reach valid length for randomBytes logic in this context
  return Number(hex);
}

/**
 * Generates an arbitrary integer from any byte array input.
 */
function generateFromByteArray(data: Uint8Array): number {
  const hex = data.toString('hex');
  // Convert the string representation of the bytes directly to a large int (since we can't easily parse raw arrays in JS without BigInt or external libs)
  return Number(hex); 
}

/**
 * Generates an arbitrary integer from any BigInt input.
 */
function generateFromBigInt(b: bigint): number {
  // We need to handle the case where b is a large BigInt (e.g., "1234567890123456789012345") and convert it back to int.
  // Since we can't parse arbitrary strings directly in JS without external libs or libraries like BigInteger, we will rely on the fact that most BigInts are small enough for hex representation (up to ~18 digits) which is handled by generateFromString/generateFromByteArray logic above via padding with zeros if necessary.
  
  // However, strictly speaking, converting a huge BigInt string 'to int' in JS without external libs is impossible and would require libraries like BigInteger.js or similar. 
  // Given the context of "generating arbitrary integers", we will assume standard JavaScript number types are sufficient for typical use cases unless specifically requested to handle non-numeric inputs (which generateFromString handles).
  
  return Number(b.toString()); 
}

/**
 * Generates an integer representing a specific value using modular arithmetic.
 */
function modInverse(n: bigint): bigint {
  // This function is derived from Fermat's Little Theorem logic adapted for arbitrary integers, but since we can't use BigInteger.js directly in this environment without external dependencies (which might be forbidden or blocked), 
  // and standard JS BigInt operations are powerful enough to handle most modular arithmetic if n > 2^531 approx.
  
  const MOD = 998244353n; // A prime number often used in cryptography for testing inverse properties
  
  let x: bigint = 0, y: bigint = 1;
  
  while (true) {
    if (x < n && !y % n === 0) break; 
    const q = Math.floor(n / x); // Integer division by x
    const r = n - (q * x); // Remainder
    
    if (!r || isNaN(r)) break;
    
    y = x;
    x = r;
  }

  return BigInt(x.toString()); 
}

/**
 * Generates a random number between min and max.
 */
function generateRandomInRange(min: number, max?: number): number {
  if (!max) return Math.floor(Math.random() * (min + 1)); // Fallback to min
  
  let attempt = 0;
  while (attempt < 256 && randomBytes(4).toString('hex').split('').map(Number)[0] !== max) {
    attempt++;
  }
  return Math.floor(randomBytes(4).toString('hex').split('').map(Number)); // Return a valid value, not
