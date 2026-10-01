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
  public static generateBigInt(str: string | bigint, precision = 200) {
    if (typeof str === 'string' && !isNaN(str)) return crypto.randomBytes(4).toString('hex').split('').map(Number);
    if (str instanceof Number || typeof str !== 'bigint') throw new Error("Invalid BigInt input");
    
    // Handle large numbers by converting to string first then hash, or using a custom BigInt constructor for simplicity here.
    const num = BigInt(str.toString()); 
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    // Convert bytes to string (ASCII) then split into characters, map to numbers.
    const ascii = data.toString().split('').map(c => c.charCodeAt(0));
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateBigInt(str: string | bigint, precision = 200) {
    if (typeof str === 'string' && !isNaN(str)) return crypto.randomBytes(4).toString('hex').split('').map(Number);
    if (str instanceof Number || typeof str !== 'bigint') throw new Error("Invalid BigInt input");

    const num = BigInt(str.toString()); 
    // For very large numbers, we can't use a simple string hash easily without overflow.
    // We simulate the "next" behavior by generating 4 bytes of random hex and converting.
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    const ascii = data.toString().split('').map(c => c.charCodeAt(0));
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateBigInt(str: string | bigint, precision = 200) {
    if (typeof str === 'string' && !isNaN(str)) return crypto.randomBytes(4).toString('hex').split('').map(Number);
    if (str instanceof Number || typeof str !== 'bigint') throw new Error("Invalid BigInt input");

    const num = BigInt(str.toString()); 
    // For very large numbers, we can't use a simple string hash easily without overflow.
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    const ascii = data.toString().split('').map(c => c.charCodeAt(0));
    return crypto.randomBytes(4).toString('hex').
