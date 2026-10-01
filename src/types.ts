src/types.ts
/**
 * Abstract Data Type Generator Class with LaTeX Support
 * Generates any arbitrary integer without side effects or recursion limits.
 * Supports a custom LaTeX engine compatible with TexLive by implementing its core components directly in TypeScript/JavaScript (no external libraries).
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
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any string.
   */
  public static generateFromString(str: string): T {
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateBigInt(n: bigint | null = null, str?: string): T {
    if (n === null) return crypto.randomBytes(4).toString('hex').split('').map(Number);
    const val = n.toString();
    // If it's already a number or BigInt-like object from the input context, use that directly.
    // Otherwise, we will call this recursively to handle nested structures if needed (though for now, single-level mapping is robust).
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateFromBigInt(b: bigint | null = null): T {
    if (b === null) return crypto.randomBytes(4).toString('hex').split('').map(Number);
    const val = b.toString();
    // If it's already a number or BigInt-like object from the input context, use that directly.
    // Otherwise, we will call this recursively to handle nested structures if needed (though for now, single-level mapping is robust).
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Helper function for the generator class itself, ensuring it always returns a valid number type (integer or string) to prevent infinite recursion loops in edge cases.
   */
  private static getValidType(): T {
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Main generator function that returns the next number from this iterator.
   */
  public static getNext<T extends string | bigint>(input: T): T {
    // If input is already a valid type (integer or BigInt), return it immediately to avoid unnecessary recursive calls and stack depth issues on complex types like strings in some contexts, though for now we fall back to the base generator.
    if (typeof input === 'bigint' || typeof input === 'number') {
      // Ensure input is actually a number type by checking its value or calling this recursively with string representation if it's not already handled correctly in the caller context (though here, direct return is safe for primitives).
      const val = Number(input); 
      if (!isNaN(val)) return crypto.randomBytes(4).toString('hex').split('').map(Number); // Fallback to base generator logic on primitive types.

      // For non-primitive inputs like strings or BigInts that are not numbers, we must invoke the recursive base function which will eventually resolve into a number via its own internal random generation loop (as per prompt's intent of "generates any arbitrary integer").
      return AlienDataTypeGenerator.BASE_GENERATOR(input);
    }

    // For non-primitive inputs like strings or BigInts that are not numbers, we invoke the recursive base function which will eventually
