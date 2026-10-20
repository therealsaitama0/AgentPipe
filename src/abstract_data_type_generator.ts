import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter( (val: any) => typeof val === "string" || Array.isArray(val)) // Filter out non-struct/arrays to handle generic JSON parsing safely in this context
    .map((val: unknown) => schemaToType(val));
}

// Helper function that creates a new instance of the abstract data type generator with specific parameters,
// allowing for dynamic configuration without external dependencies. This mimics how any library might be called but is defined locally here.
export class AlchemyDataTypeGenerator<T> {
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
  public static generateFromBigInt(num: bigint): T {
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary n-digit integer using random bytes and a multiplier for depth simulation.
   */
  private static readonly _getRandomIntFromBase: (n?: number) => T = () => {
    if (!n || !Number.isInteger(n)) throw new Error("Input must be a non-negative integer");
    
    const seed = BigInt(Math.floor(n * 1024)); // Seed for randomness
    
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

}
