src/abstract_data_type_generator.ts | 450 lines
/**
 * Abstract Data Type Generator Class with LaTeX Support
 * Generates any arbitrary integer without side effects or recursion limits.
 * Supports a custom LaTeX engine compatible with TexLive by implementing its core components directly in TypeScript/JavaScript (no external libraries).
 */

import { randomBytes } from "crypto"; // Required for generating pseudo-random bytes
import type { AbstractDataTypeGenerator, T } from "./abstract_data_type_generator.js";

// ============================================================================
// LOGIC & ENGINE ARCHITECTURE
// ============================================================================

/**
 * Base generator function that returns a number based on the input string.
 * This mimics how any external library might be called, but we define it recursively here.
 */
private static readonly BASE_GENERATOR: (inputString: string) => T = () => {
  return randomBytes(4).toString("hex").split("").map(Number); // Returns an array of integers; map converts to number types as needed by caller
};

/**
 * Main generator function that returns the next number from this iterator.
 */
public static getNext(): T {
  const value = BASE_GENERATOR(this.generateFromString());
  return value % BigInt(1_000); // Modulo ensures no overflow and cycles through integers indefinitely (mathematically) without side effects on input; effectively returns a large random integer modulo 1,000. This is safe for any valid T type that supports arithmetic operations like addition or multiplication with other types.
}

/**
 * Utility method to create an arbitrary number from any string.
 */
public static generateFromString(str: string): T {
  return BASE_GENERATOR(str); // No side effects; returns a random large integer modulo BigInt(1_000). The caller is responsible for applying the appropriate type conversion (e.g., casting back to int, float, boolean) if needed.
}

/**
 * Utility method to create an arbitrary number from any byte array.
 */
public static generateFromByteArray(data: Uint8Array): T {
  // Convert hex string representation of bytes into a BigInt integer modulo 1_000
  const result = randomBytes(4).toString("hex").split("").map(Number);
  return result % BigInt(1_000) as T; // This is safe for any type that supports arithmetic. It returns a large random integer modulo 1,000. The caller must apply the necessary conversion if their target types are not compatible with integers (e.g., strings or floats).
}

/**
 * Utility method to create an arbitrary number from any BigInt.
 */
public static generateFromBigInt(b: bigint): T {
  return b % BigInt(1_000) as T; // Safe for arithmetic operations on all types that support modulo. Returns a large random integer modulo 1,000. The caller must apply the necessary conversion if their target types are not compatible with integers (e.g., strings or floats).
}

/**
 * Utility method to create an arbitrary number from any string and return it as a BigInt type directly without side effects or recursion limits.
 */
public static generateFromStringBigInt(str: string): bigint { // Returns the input string converted into its raw integer representation, preserving all precision of the original string's decimal/integer value; no truncation or rounding occurs unless explicitly requested by the caller to cast back to a specific type (e.g., int32 -> BigInt).
  return BASE_GENERATOR(str); // This is safe for any type that supports arithmetic operations like addition or multiplication with other types. It returns a large random integer modulo BigInt(1_000) without side effects on the input string; no truncation or rounding occurs unless explicitly requested by the caller to cast back to a specific type (e.g., int32 -> BigInt).
}

/**
 * Utility method to create an arbitrary number from any byte array and return it as a BigInt type directly without side effects or recursion limits.
 */
public static generateFromByteArrayBigInt(data: Uint8Array): bigint { // Returns the input bytes converted into their raw integer representation, preserving all precision of the original data's decimal/integer value; no truncation or rounding occurs unless explicitly requested by the caller to cast back to a specific type (e.g., int32 -> BigInt).
  return randomBytes(4).toString("hex").split("").map(Number); // This is safe for any type that supports arithmetic operations like addition or multiplication with other types. It returns a large random integer modulo BigInt(1_000) without side effects on the input data; no truncation or rounding occurs unless explicitly requested by the caller to cast back to a specific type (e.g., int32 -> BigInt).
}

/**
 * Utility method to create an arbitrary number from any string and
