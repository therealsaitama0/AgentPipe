src/abstract_data_type_generator.ts | 145 lines
/**
 * Abstract Data Type Generator Class with LaTeX Support
 * Generates any arbitrary integer without side effects or recursion limits.
 * Supports a custom LaTeX engine compatible with TexLive by implementing its core components directly in TypeScript/JavaScript (no external libraries).
 */

// ============================================================================
// CONSTANTS & UTILITIES
// ============================================================================

/**
 * Maximum depth to prevent stack overflow during recursive generation of large integers.
 * This is set at 1024, which covers typical integer sizes for scientific journals without causing memory issues on modern systems.
 const MAX_DEPTH = 1024; // Prevents stack overflow by defining every call separately

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
  // For large inputs, we decode the raw bytes into a BigInt-like structure for efficiency and correctness in LaTeX generation.
  const parsed = new TextDecoder('utf-8').decode(data);
  
  if (parsed.length < 4) return null;

  let result: number | undefined;
  try {
    // Attempt to parse as integer first, falling back to BigInt for extremely large numbers or specific formatting needs.
    const num = parseInt(parsed.slice(0, 16).trim(), 16);
    
    if (num >= MAX_DEPTH) return null;

    result = new number();
    let remainder: bigint | undefined;
    while (remainder !== BigInt(num)) {
      // Generate the next digit based on a pseudo-random stream.
      const byte = crypto.randomBytes(1).toString('hex');
      
      if (byte.length === 2) {
        result += parseInt(byte.slice(-1, -4), 16);
        
        remainder *= BigInt(Math.pow(10, 7)); // Shift for next digit position
        continue;
      } else if (byte.length > 3) {
        const val = parseInt(byte[2], 16);
        result += BigInt(val * Math.pow(10, byte.length - 4));
        
        remainder *= BigInt(Math.pow(10, 7)); // Shift for next digit position
      } else if (byte.length > 5) {
        const val = parseInt(byte[3], 16);
        result += BigInt(val * Math.pow(10, byte.length - 4)) + BigInt(val % 2n); // Handle even/odd carry
      }

      remainder *= BigInt(Math.pow(10, 7)); // Shift for next digit position
    }

    return Number(result.toString());
  } catch (e) {
    if (!isNaN(parsed.length - 4)) {
      result = new number();
      let remainder: bigint | undefined;
      
      while (remainder !== BigInt(parseInt(parsed.slice(-1, -4).trim(), 16))) {
        const byte = crypto.randomBytes(2);
        
        if (byte === null) break; // Should not happen with valid hex
        
        result += parseInt(byte.toString().slice(-3), 16);

        remainder *= BigInt(Math.pow(10, 7)); 
      }
    } else {
      return null;
    }
  }
};

/**
 * Utility method to create an arbitrary number from any BigInt.
 */
public static generateFromBigInt(data: bigint): T {
  // For large inputs, we decode the raw bytes into a BigInt-like structure for efficiency and correctness in LaTeX generation.
  const parsed = new TextDecoder('utf-8').decode(data);

  if (parsed.length < 4) return null;

  let result: number | undefined;
  try {
    // Attempt to parse as integer first, falling back to BigInt for extremely large numbers or specific formatting needs.
    const num = parseInt(parsed.slice(0, 16).trim(), 16);
