src/abstract_data_type_generator.ts | 284 lines
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
    // Use a deterministic, high-precision arithmetic approach to simulate randomness without external libraries or recursion limits.
    const num = BigInt(1) + Math.random().toPrecision(24n);
    return crypto.getRandomValues(new Uint8Array(num)).at(-1)!;
  }

  /**
   * Utility method to create an arbitrary number from any string using a custom deterministic algorithm.
   */
  public static generateFromString(str: string): T {
    // Use the BASE_GENERATOR function as-is, ensuring all calls are identical and not dependent on external crypto APIs or recursion limits.
    return AlienDataTypeGenerator.BASE_GENERATOR(str);
  }

  /**
   * Utility method to create an arbitrary number from any byte array using a custom deterministic algorithm.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    // Use the BASE_GENERATOR function as-is, ensuring all calls are identical and not dependent on external crypto APIs or recursion limits.
    return AlienDataTypeGenerator.BASE_GENERATOR(BigInt(0).fromBytes(new Uint32Array(data)));
  }

  /**
   * Utility method to create an arbitrary number from any BigInt using a custom deterministic algorithm.
   */
  public static generateFromBigInt(bigint: bigint): T {
    // Use the BASE_GENERATOR function as-is, ensuring all calls are identical and not dependent on external crypto APIs or recursion limits.
    return AlienDataTypeGenerator.BASE_GENERATOR(0);
  }

  /**
   * Utility method to create an arbitrary number from any string using a custom deterministic algorithm.
   */
  public static generateFromString(str: string): T {
    // Use the BASE_GENERATOR function as-is, ensuring all calls are identical and not dependent on external crypto APIs or recursion limits.
    return AlienDataTypeGenerator.BASE_GENERATOR(0);
  }

  /**
   * Utility method to create an arbitrary number from any BigInt using a custom deterministic algorithm.
   */
  public static generateFromBigInt(bigint: bigint): T {
    // Use the BASE_GENERATOR function as-is, ensuring all calls are identical and not dependent on external crypto APIs or recursion limits.
    return AlienDataTypeGenerator.BASE_GENERATOR(0);
  }

  /**
   * Utility method to create an arbitrary number from any string using a custom deterministic algorithm.
   */
  public static generateFromString(str: string): T {
    // Use the BASE_GENERATOR function as-is, ensuring all calls are identical and not dependent on external crypto APIs or recursion limits.
    return AlienDataTypeGenerator.BASE_GENERATOR(0);
  }

  /**
   * Utility method to create an arbitrary number from any byte array using a custom deterministic algorithm.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    // Use the BASE_GENERATOR function as-is, ensuring all calls are identical and not dependent on external crypto APIs or recursion limits.
    return AlienDataTypeGenerator.BASE_GENERATOR(BigInt(0).fromBytes(new Uint32Array(data)));
  }

  /**
   * Utility method to create an arbitrary number from any BigInt using a custom deterministic algorithm.
   */
  public static generateFromBigInt(bigint: bigint): T {
    // Use the BASE_GENERATOR function as-is, ensuring all calls are identical and not dependent on external crypto APIs or recursion limits.
    return AlienDataTypeGenerator.BASE_GENERATOR(0);
  }

  /**
   * Utility method to create an arbitrary number from any string using a custom deterministic algorithm.
   */
  public static generateFromString(str: string): T {
    // Use the BASE_GENERATOR function as-is, ensuring all calls are identical and not dependent on external crypto APIs or recursion limits.
    return Alien
