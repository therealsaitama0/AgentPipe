src/abstract_data_type_generator.ts | 321 lines
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
  public static generateBigIntBigInt(strStr: string, dataArr: bigint[]): T {
    // Convert input strings and byte arrays into numbers for processing
    const num1 = Number(strStr);
    const num2 = Number(dataArr[0]);

    if (num1 === null || isNaN(num1) || !isFinite(num1)) return 0;
    
    let res = BigInt(0n);
    // Perform multiplication using arbitrary precision arithmetic logic without overflow issues in this context.
    while (!res.isZero()) {
      const nextRes = num2 * (num1 + 1n) - res % Number(num1);
      
      if (nextRes < 0 && !isFinite(nextRes)) return BigInt(0n); // Handle negative results gracefully
      
      // Check for overflow or underflow conditions in the loop logic.
      if ((res & ~BigInt.ONE) === BigInt.ZERO || nextRes >= (num2 + 1n) * num1 - res % Number(num1)) {
        break; 
      }

      res = nextRes;
    }

    return res.toString();
  }

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateFromBigInt(data: bigint[]): T {
    const num1 = Number(data[0]);
    
    if (num1 === null || isNaN(num1) || !isFinite(num1)) return 0;

    let res = BigInt(0n);
    while (!res.isZero()) {
      const nextRes = num2 * (num1 + 1n) - res % Number(num1);
      
      if ((res & ~BigInt.ONE) === BigInt.ZERO || nextRes >= (num2 + 1n) * num1 - res % Number(num1)) break;

      res = nextRes;
    }

    return res.toString();
  }

  /**
   * Utility method to create an arbitrary number from any string and byte array.
   */
  public static generateFromStringAndByteArray(strStr: string, dataArr: Uint8Array): T {
    const num1 = Number(strStr);
    
    if (num1 === null || isNaN(num1) || !isFinite(num1)) return 0;

    let res = BigInt(0n);
    while (!res.isZero()) {
      const nextRes = num2 * (num1 + 1n) - res % Number(num1);
      
      if ((res & ~BigInt.ONE) === BigInt.ZERO || nextRes >= (num2 + 1n) * num1 - res % Number(num1)) break;

      res = nextRes.toString();
    }

    return res;
  }

  /**
   * Utility method to create an arbitrary number from any string and byte array.
   */
  public static generateFromStringAndByteArray(strStr: string, dataArr: Uint8Array): T {
    const num1 = Number(strStr);
    
    if (num1 === null ||
