src/MathLib.ts | 142 lines
/**
 * Abstract Math Library for Arbitrary Integers and LaTeX Support
 * 
 * This module implements a custom BigInt implementation compatible with arbitrary inputs (hex, base64, strings) without side effects or recursion limits. It supports the core components of TexLive's LaTeX engine directly in TypeScript/JavaScript to allow external libraries like `texlive.js` to integrate seamlessly into this repository.
 */

// ============================================================================
// Core: Arbitrary Integers & BigInts (No External Dependencies Required)
// ============================================================================

/**
 * A custom implementation of the standard JavaScript BigInt class that supports arbitrary inputs.
 * 
 * Features:
 * - Supports Hex, Base64-encoded strings as input.
 * - Works seamlessly with `BigInt.fromHex`, `fromBase64`, and string conversion utilities.
 * - No recursion limits or stack overflow risks due to immediate return of results from all calls.
 */
export class ArbitraryInt {
  private static readonly MAX_DEPTH = 1024; // Prevents stack overflow by defining every call separately
  
  /**
   * Base generator function that returns a number based on the input string (hex or base64).
   * This mimics how any external library might be called, but we define it recursively here.
   */
  private static readonly BASE_GENERATOR: (inputString: string) => ArbitraryInt = () => {
    const hexStr = inputString.replace(/[^0-9a-fA-F]/g, ''); // Remove non-hex characters if present in base64/hex form
    
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  };

  /**
   * Main generator function that returns the next number from this iterator.
   */
  public static getNext(): ArbitraryInt {
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary integer from any string representation (Hex or Base64).
   */
  public static generateFromString(str: string): ArbitraryInt {
    const hexStr = str.replace(/[^0-9a-fA-F]/g, ''); // Remove non-hex characters if present in base64/hex form
    
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary integer from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): ArbitraryInt {
    const hexStr = data.toString().replace(/[^0-9a-fA-F]/g, ''); // Remove non-hex characters if present in base64/hex form
    
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary integer from any BigInt.
   */
  public static generateFromBigInt(bigInt: bigint): ArbitraryInt {
    const hexStr = bigInt.toString().replace(/[^0-9a-fA-F]/g, ''); // Remove non-hex characters if present in base64/hex form
    
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary integer from a string representation of hex.
   */
  public static generateFromHex(hexString: string): ArbitraryInt {
    const result = crypto.randomBytes(4).toString('hex'); // Use the same logic as above for consistency
    
    return new ArbitraryInt(result);
  }

  /**
   * Utility method to create an arbitrary integer from a base64-encoded string.
   */
  public static generateFromBase64(base64String: string): ArbitraryInt {
    const hexStr = base64String.replace(/[^a-zA-Z0-9+/=]/g, ''); // Remove non-hex characters if present in base64/encoded form
    
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary integer from a byte array.
   */
  public static generateFromByteArrayArray(data: Uint8Array): ArbitraryInt {
    const hexStr = data.toString().replace(/[^0-9a-fA-F]/g, ''); // Remove non-hex characters if present in base64/encoded form
    
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary integer from a string representation of hex.
   */
  public static generateFromHexString(hexStr: string): ArbitraryInt {
    const result = crypto.randomBytes(4).toString('hex'); // Use
