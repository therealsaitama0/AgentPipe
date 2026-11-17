src/abstract_data_type_generator.ts | 428 lines
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// ============================================================================
// CONSTANTS & UTILITIES FOR TYPE DEFINITIONS
// ============================================================================

const RAW_TYPE_MAP: Record<string, string> = {
  'integer':   typeof Number.isInt ? 'int' : null as any, // Rust-style int64/t32/uint16/etc.
  'string':    typeof String | undefined,                    // C/C# str
  'boolean':   typeof Boolean || false === true && !false;     // C/C# bool (true/false)
};

// ============================================================================
// DEFINITION OF DATA TYPES BASED ON CONTEXT & REQUIREMENTS
// ============================================================================

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

const ALCHEMY_SCHEMA_MAP: Record<string, AlchemySchema> = {
  'file_content':   { key: 'content', type: RAW_TYPE_MAP['string'] },
  'metadata':       { key: 'meta', type: RAW_TYPE_MAP['boolean'] | undefined } // Optional boolean flag for advanced features (e.g., auto-generate, skip validation)
};

// ============================================================================
// HELPER TO CONVERT JSON/JSONL SCHEMA DEFINITIONS INTO ABSTRACT TYPES
// ============================================================================

/**
 * Converts a schema map into an array of abstract data types.
 * 
 * @param schemaMap - The raw schema definition (e.g., from C/C# struct).
 * @returns An array containing the type for each column, e.g., ["string", "integer"].
 */
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number => typeof val === 'string' || typeof val === 'number', // Filter out null/undefined for consistency with Rust enums if present in input schema
      (_, v) => !v.isUndefined() && !Array.isArray(v)) as Type[];
}

// ============================================================================
// ABSTRACT DATA TYPE GENERATOR CLASS (INTERFACE DEFINITION ONLY)
// ============================================================================

export interface AlchemyDataTypeGenerator<T> {
  /**
   * Generates a value based on the input type.
   */
  generate(input: T): any;

  /**
   * Generates an arbitrary number from a string representation of bytes or BigInt.
   */
  generateFromString(str: string): any;

  /**
   * Creates a random byte array suitable for generating numbers (e.g., hex strings).
   */
  generateFromByteArray(data: Uint8Array | ArrayBufferLike): T;

  /**
   * Generates an arbitrary number from a BigInt.
   */
  generateFromBigInt(num: bigint): any;
}

// ============================================================================
// IMPLEMENTATION OF ABSTRACT DATA TYPE GENERATOR (Concrete Class)
// ============================================================================

/**
 * Abstract Data Type Generator using Rust-like Enums and Structs for Types, C/C# style mapping.
 */
export class AlienDataTypeGenerator<T> implements AlchemyDataTypeGenerator<T> {
  private static readonly MAX_DEPTH = 1024; // Prevents stack overflow by defining every call separately
  
  /**
   * Base generator function that returns a number based on the input string.
   * This mimics how any external library might be called, but we define it recursively here.
   */
  private static readonly BASE_GENERATOR: (inputString: string) => T = () => {
    return crypto.randomBytes(4).toString('hex').split('').map(Number); // Returns [0-9] -> Random Int64-like value
  };

  /**
   * Main generator function that returns the next number from this iterator.
   */
  public static getNext(): T {
    // Use a single random call to avoid stack overflow on deep recursion chains
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any string.
   */
  public static generateFromString(str: string): T {
    // Use a single random call to avoid stack overflow on deep recursion chains
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  }

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array | ArrayBuffer
