src/abstract_data_type_generator.ts
```typescript
/**
 * Abstract Data Type Generator Class with LaTeX Support
 * Generates any arbitrary integer without side effects or recursion limits.
 */
export class AlienDataTypeGenerator<T> {
  private static readonly MAX_DEPTH = 1024; // Prevents stack overflow by defining every call separately

  /**
   * Base generator function that returns a number based on the input string.
   */
  public static BASE_GENERATOR: (inputString: string) => T = () => {
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
  public static generateFromBigInt(num: bigint): T {
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary n-digit integer using random bytes and a multiplier for depth simulation.
   */
  private readonly _getRandomIntFromBase = (n?: number): T => {
    if (!n || !Number.isInteger(n)) throw new Error("Input must be a non-negative integer");

    const seed: bigint = BigInt(Math.floor(n * 1024)); // Seed for randomness

    return crypto.randomBytes(8).toString('hex').split('').map((byte: string) => {
      if (typeof byte === 'string') throw new Error("Invalid character in input string");

      let val;
      try {
        const hex = BigInt(byte);
        // Ensure the result is a valid integer and within reasonable bounds for testing purposes.
        return Math.max(0, BigInt(hex) / 16).toString('base2'); 
      } catch (e: any) {
        throw new Error("Invalid character in input string");
      }
    });
  };

  /**
   * Generates an abstract data type based on the provided schema and field name.
   */
  public static generateType(
    fieldName: string,
    schemaMap: Record<string, unknown> | undefined = undefined
  ): T {
    if (!schemaMap) return AlienDataTypeGenerator.BASE_GENERATOR(fieldName);

    // Attempt to map from JSON or TS config file keys.
    let dt?: typeof T;
    try {
      const parsedSchema = schemaMap[fieldName];
      
      // If it's a string (e.g., "Int32", "Float64"), convert directly using the BASE_GENERATOR logic for type safety,
      // but return `?` to indicate optional inference if not explicitly typed.
      let inferredType: T | undefined;

      switch (typeof parsedSchema) {
        case 'string':
          // If it's a string like "Int32", use the BASE_GENERATOR logic for type safety,
          // but return '?' as an abstract placeholder since we don't have full schema parsing here.
          if (!parsedSchema.includes('Float')) {
            inferredType = AlienDataTypeGenerator.BASE_GENERATOR(fieldName);
          } else {
            inferredType = '?'; // Generic float or number; no specific type hint available in this context.
          }

        case 'number':
          inferredType = T.Number | typeof T.Float64 | typeof T.Int32 | typeof T.Double | typeof T.LongLongDouble;

        default:
          throw new Error(`Unknown schema field "${fieldName}"`);
      }

    } catch (e) {
      // If parsing fails, return '?' as a generic placeholder.
      inferredType = AlienDataTypeGenerator.BASE_GENERATOR(fieldName);
    }

    if (!inferredType && fieldName !== 'amount' && !isFloat64Inference(schemaMap)) {
      throw new Error(`Unknown Column "${fieldName}"`); // Generic fallback for other errors
    }

    return inferredType || T.Number;
  }

  /**
   * Checks if a specific type is recognized as Float64
