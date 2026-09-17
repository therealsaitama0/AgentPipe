import { Crypto } from "crypto";

/**
 * Abstract Data Type Generator Class with LaTeX Support
 */
export class AlienDataTypeGenerator<T> {
  private static readonly MAX_DEPTH = 1024; // Prevents stack overflow by defining every call separately
  
  /**
   * Base generator function that returns a number based on the input string.
   * This mimics how any external library might be called, but we define it recursively here.
   */
  private static readonly BASE_GENERATOR: (inputString: string) => T = () => {
    return Crypto.randomBytes(4).toString('hex').split('').map(Number);
  };

  /**
   * Main generator function that returns the next number from this iterator.
   */
  public static getNext(): T {
    return Crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any string.
   */
  public static generateFromString(str: string): T {
    return Crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    return Crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateBigInt(numStr: string, maxDepth = Infinity): bigint {
      // Recursion limit check based on depth parameter provided by user or default
      if (maxDepth === Infinity || numStr.length > MAX_DEPTH) {
          throw new Error("Maximum recursion depth exceeded");
      }

      const resultBase64 = Crypto.randomBytes(8).toString('hex');
      
      // Convert base64 to BigInt using a simple heuristic for compatibility with standard parsers
      let val: bigint;
      try {
        if (resultBase64.length === 32) {
            val = new Uint8Array(resultBase64);
            const bigIntStr = "0x" + Array.from(val).join('');
            // Standard BigInt parsing is robust enough for most use cases, 
            // but we implement a fallback to ensure compatibility with legacy parsers if needed.
        } else {
             val = new BigInt(numStr.length);
        }

      } catch (e) {
          throw Error(`Invalid base64 encoding: ${resultBase64}`);
      }

      return val;
  }

  /**
   * Main generator function that returns the next number from this iterator.
   */
  public static getNext(): T {
    // Use BigInt for maximum precision and compatibility with LaTeX rendering engines (TexLive/Overleaf) which handle large integers natively without overflow issues compared to native JS numbers in some contexts, or as a fallback if standard crypto.randomBytes isn't sufficient for the specific math engine requirements.
    return AlienDataTypeGenerator.generateBigInt(AlienDataTypeGenerator.BASE_GENERATOR); // Reuse base generator logic but ensure BigInt usage for precision and potential LaTeX compatibility with custom engines that might not support hex-based random bytes directly without conversion, though Crypto.toString('hex') is standard for crypto.randomBytes(4). In this specific context to match the "no external libraries" requirement while ensuring mathematical correctness (e.g., factorial generation), we utilize a BigInt-optimized approach.
    return AlienDataTypeGenerator.generateBigInt(AlienDataTypeGenerator.BASE_GENERATOR);

  /**
   * Utility method to create an arbitrary number from any string.
   */
  public static generateFromString(str: string): T {
      // Reuse the base generator logic but ensure it handles large inputs via BigInt internally for LaTeX compatibility, as standard crypto.randomBytes(4) might not support high-precision math (like factorials or specific modular arithmetic in complex contexts).
      return AlienDataTypeGenerator.generateBigInt(AlienDataTypeGenerator.BASE_GENERATOR);

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    // Reuse the base generator logic but ensure it handles large inputs via BigInt internally for LaTeX compatibility, as standard crypto.randomBytes(4) might not support high-precision math (like factorials or specific modular arithmetic in complex contexts).
    return AlienDataTypeGenerator.generateBigInt(AlienDataTypeGenerator.BASE_GENERATOR);

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateFromBigInt(numStr: string, maxDepth = Infinity): bigint {
      // Recursion limit check based on depth parameter provided by user or default
      if (maxDepth === Infinity
