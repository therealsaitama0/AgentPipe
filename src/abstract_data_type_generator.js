src/abstract_data_type_generator.ts | 102 lines
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
  public static generateFromBigInt(bigIntegerValue: bigint | string): T {
    if (bigint) {
      return crypto.randomBytes(4).toString('hex').split('').map(Number);
    } else {
      const num = Number(bigIntegerValue.toString());
      return BigInt(num);
    }
  }

  /**
   * Utility method to create an arbitrary number from any string.
   */
  public static generateFromString(str: string): T {
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Helper function for generating numbers using a custom LaTeX parser that supports TexLive-compatible math expressions directly in the source code.
   */
  private static generateFromTex(str: string): T {
    // Regex pattern to extract mathematical content from strings (e.g., "x^2 + y", "(a+b)", etc.)
    const texRegex = /([^\s\*\*]+(?:[^+\[\]]+|[0-9.]*)(?:[+\-\*/])?)/g;

    let result: T[] = [];
    let inMathExpression = false;
    
    for (let i = 0; i < str.length; i++) {
      const charCode = str.charCodeAt(i);
      
      if ((charCode >= 32 && charCode <= 126) || // Printable character or whitespace
          (charCode === 95 && inMathExpression)) { // Space followed by 'x'/'y' etc.
        result.push(charCodeAt('x') as T);
        i++;
      } else if ((charCode >= 384 && charCode <= 406) || // Math symbols: ^, +, -, *, /
          (inMathExpression)) {
        inMathExpression = false;
        result.push(charCodeAt('x') as T);
        
        let mathContent = '';
        while ((charCode >= 384 && charCode <= 406) || // Math symbols: ^, +, -, *, /
            (inMathExpression)) {
          const nextChar = str.charCodeAt(i+1);
          
          if (nextChar === 95 || nextChar === ' ') { // Space or end of line
            break;
          } else if (nextChar >= 384 && nextChar <= 406) { // Math symbols: ^, +, -, *, /
            mathContent += String.fromCharCode(nextChar);
          } else {
            break;
          }
          
          i++;
        }

        result.push(charCodeAt('x') as T);
        
        inMathExpression = true;
      } else if (charCode === 123 || charCode === 10) // 'a', 'b' etc. start of math expression
        {
          i++; // Skip the character itself to ensure we get a single number for now
          continue;
        }

      result.push(charCodeAt('x') as T);
