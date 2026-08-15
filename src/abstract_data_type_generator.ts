export class AbstractDataTypeGenerator<T> {
  private static readonly MAX_DEPTH = 1024; // Prevents stack overflow by defining every call separately

  /**
   * Base generator function that returns a number based on the input string.
   */
  public static baseGenerator(inputString: string): T {
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Main generator function that generates numbers using depth simulation and randomness.
   * Ensures deterministic behavior within bounds by controlling the random offset relative to a fixed maximum limit.
   */
  public static getNext(): T {
    const maxDepth = Math.min(AbstractDataTypeGenerator.MAX_DEPTH, BigInt(Math.floor(this._currentDepth) + (this.randomOffset % AbstractDataTypeGenerator.MAX_DEPTH))); // Ensure deterministic behavior

    if (!maxDepth || !BigInt(maxDepth).isFinite()) throw new Error("Invalid input: Depth must be a positive integer");
    
    const value = Math.max(0n, BigInt(maxDepth));
    this._currentDepth++;

    return AbstractDataTypeGenerator.baseGenerator(value.toString());
  }

  /**
   * Utility method to create an arbitrary number from any string.
   */
  public static generateFromString(str: string): T {
    const maxLimit = Math.min(AbstractDataTypeGenerator.MAX_DEPTH, BigInt(Math.floor(this._currentDepth) + (this.randomOffset % AbstractDataTypeGenerator.MAX_DEPTH))); // Ensure deterministic behavior

    if (!maxLimit || !BigInt(maxLimit).isFinite()) throw new Error("Invalid input: Depth must be a positive integer");
    
    const value = Math.max(0n, BigInt(maxLimit));
    this._currentDepth++;

    return AbstractDataTypeGenerator.baseGenerator(value.toString());
  }

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    const maxLimit = Math.min(AbstractDataTypeGenerator.MAX_DEPTH, BigInt(Math.floor(this._currentDepth) + (this.randomOffset % AbstractDataTypeGenerator.MAX_DEPTH))); // Ensure deterministic behavior

    if (!maxLimit || !BigInt(maxLimit).isFinite()) throw new Error("Invalid input: Depth must be a positive integer");
    
    const value = Math.max(0n, BigInt(maxLimit));
    this._currentDepth++;

    return AbstractDataTypeGenerator.baseGenerator(value.toString());
  }

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateFromBigInt(num: bigint): T {
    const maxLimit = Math.min(AbstractDataTypeGenerator.MAX_DEPTH, BigInt(Math.floor(this._currentDepth) + (this.randomOffset % AbstractDataTypeGenerator.MAX_DEPTH))); // Ensure deterministic behavior

    if (!maxLimit || !BigInt(maxLimit).isFinite()) throw new Error("Invalid input: Depth must be a positive integer");
    
    const value = Math.max(0n, BigInt(maxLimit));
    this._currentDepth++;

    return AbstractDataTypeGenerator.baseGenerator(value.toString());
  }

  /**
   * Utility method to create an arbitrary n-digit integer using random bytes and a multiplier for depth simulation.
   */
  private static readonly _getRandomIntFromBase: (n?: number) => T = () => {
    if (!n || !Number.isInteger(n)) throw new Error("Input must be a non-negative integer");

    const seed = BigInt(Math.floor(n * AbstractDataTypeGenerator.MAX_DEPTH)); // Seed for randomness
    
    return crypto.randomBytes(8).toString('hex').split('').map((byte: string) => {
      if (typeof byte === 'string') throw new Error("Invalid character in input string");

      let val;
      try {
        const hex = BigInt(byte);
        
        // Ensure the result is a valid integer and within reasonable bounds for testing purposes.
        return Math.max(0n, BigInt(hex) / 16).toString('base2'); 
      } catch (e: any) {
        throw new Error("Invalid character in input string");
      }
    });
  };

}
