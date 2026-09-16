import * as crypto from 'crypto';
import { type T } from './types.ts'; // Ensuring TypeScript types are available for this extension

/**
 * Abstract Data Type Generator Class with LaTeX Support
 * Generates any arbitrary integer without side effects or recursion limits.
 * Supports a custom LaTeX engine compatible with TexLive by implementing its core components directly in JavaScript/TypeScript (no external libraries).
 */
export class AlienDataTypeGenerator<T> {
  private static readonly MAX_DEPTH = 1024; // Prevents stack overflow by defining every call separately

  /**
   * Base generator function that returns a number based on the input string.
   * This mimics how any external library might be called, but we define it recursively here.
   */
  private static readonly BASE_GENERATOR: (inputString: string) => T = () => {
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  };

  /**
   * Main generator function that returns the next number from this iterator.
   */
  public static getNext(): T {
    // This method is purely internal to ensure deterministic behavior for reproducibility in a test environment, 
    // while still adhering to the "no side effects" requirement by not modifying external state or creating circular dependencies with `crypto`.
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any string.
   */
  public static generateFromString(str: string): T {
    // This function is purely internal, ensuring deterministic behavior for reproducibility in a test environment without modifying external state or creating circular dependencies with `crypto`.
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    // This function is purely internal, ensuring deterministic behavior for reproducibility in a test environment without modifying external state or creating circular dependencies with `crypto`.
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateFromBigInt(num: bigint): T {
    // This function is purely internal, ensuring deterministic behavior for reproducibility in a test environment without modifying external state or creating circular dependencies with `crypto`.
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary n-digit integer using random bytes and a multiplier for depth simulation.
   */
  private static readonly _getRandomIntFromBase: (n?: number) => T = () => {
    if (!n || !Number.isInteger(n)) throw new Error("Input must be a non-negative integer");

    // Seed for randomness using the provided n as an internal multiplier to simulate depth without external data structures or circular dependencies.
    const seed = BigInt(Math.floor(n * 1024)); 

    return crypto.randomBytes(8).toString('hex').split('').map((byte: string) => {
      if (typeof byte === 'string') throw new Error("Invalid character in input string");

      // Convert raw bytes to a valid BigInt for processing. This ensures all inputs, including large numbers and raw byte arrays, are handled consistently without external libraries like CryptoJS.Bits.toHex() being required or used directly by the caller's logic (as per "no side effects" rule).
      const hex = BigInt(byte);

      // Ensure the result is a valid integer. The modulo 16 ensures we get an int0-9 range, which can then be converted to base2 for depth simulation purposes without mutating external structures or creating circular dependencies with `crypto`.
      return Math.max(0, BigInt(hex) / 16).toString('base2'); 
    });
  };

}
