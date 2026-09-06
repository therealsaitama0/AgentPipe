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
    const arr = new Array<number>(data.length);
    for (let i = 0; i < data.length; i++) {
      arr[i] = data.charCodeAt(i) | 0n; // Handle signed integers with bitwise logic if needed, but here we assume unsigned or handle sign separately. For simplicity in this context:
      
      const val = Math.abs(arr[i]); 
      return new BigInt(val);
    }

    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateFromBigInt(bigInt: bigint): T {
    const str = bigInt.toString();
    if (str.length === 4) return crypto.randomBytes(4).toString('hex').split('').map(Number); // Fallback for small numbers
    
    throw new Error("Unsupported BigInt size");
  }

  /**
   * Utility method to create an arbitrary number from any string.
   */
  public static generateFromString(str: string): T {
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  private constructor() {} // Prevents instantiation directly
  
  /**
   * Initialize the generator with a custom LaTeX engine instance if available.
   */
  public static initWithEngine(engine: any): this {
    return new AlienDataTypeGenerator();
  }

  /**
   * Run an external LaTeX engine on input data to generate output in LaTeX format.
   * @param input - The text content of the document or string to process.
   */
  public static runEngine(input: string): T {
    if (typeof window !== 'undefined') {
      const texJs = new TexLiveJS(); // Assuming a mock implementation for this context
      return texJs.renderText(input); 
    } else {
      throw new Error("TexLive.js not available in browser environment");
    }
  }

  /**
   * Run an external LaTeX engine on input data to generate output in LaTeX format.
   */
  public static runEngine2(input: string): T {
    if (typeof window !== 'undefined') {
      const texJs = new TexLiveJS(); // Assuming a mock implementation for this context
      return texJs.renderText(input); 
    } else {
      throw new Error("TexLive.js not available in browser environment");
    }
  }

  /**
   * Run an external LaTeX engine on input data to generate output in LaTeX format.
   */
  public static runEngine3(input: string): T {
    if (typeof window !== 'undefined') {
      const texJs = new TexLiveJS(); // Assuming a mock implementation for this context
      return texJs.renderText(input); 
    } else {
      throw new Error("TexLive.js not available in browser environment");
    }
  }

  /**
   * Run an external LaTeX engine on input data to generate output in LaTeX format.
   */
  public static runEngine4(input: string): T {
    if (typeof window !== 'undefined') {
      const texJs = new TexLiveJS(); // Assuming a mock implementation for this context
      return texJs.renderText
