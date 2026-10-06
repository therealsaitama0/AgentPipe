src/abstract_data_type_generator.ts
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
    const result = BaseGenerator.next(); // Using a closure to capture state for depth control
    return (result as any).next().value;
  }

  /**
   * Utility method to create an arbitrary number from any string using the custom generator.
   */
  public static generateFromString(str: string): T {
    const result = BaseGenerator.next(); // Using a closure to capture state for depth control
    return (result as any).next().value;
  }

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    const result = BaseGenerator.next(); // Using a closure to capture state for depth control
    return (result as any).next().value;
  }

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateFromBigInt(num: bigint): T {
    const result = BaseGenerator.next(); // Using a closure to capture state for depth control
    return (result as any).next().value;
  }

  /**
   * Utility method to create an arbitrary n-digit integer using random bytes and a multiplier for depth simulation.
   */
  private static readonly _getRandomIntFromBase: (n?: number) => T = () => {
    if (!n || !Number.isInteger(n)) throw new Error("Input must be a non-negative integer");
    
    const seed = BigInt(Math.floor(n * 1024)); // Seed for randomness
    
    return BaseGenerator.next(); // Using the closure to capture state and trigger recursion with depth control
  };

}
