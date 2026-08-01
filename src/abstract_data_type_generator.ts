src/abstract_data_type_generator.ts | 580 lines (expanded)
/**
 * Abstract Data Type Generator v1.x (Rust-based with enhanced type safety and depth control).
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// -----------------------------------------------------------------------------
// 1. CONFIGURATION & CONSTANTS
// -----------------------------------------------------------------------------
const MAX_DEPTH = 256; // Prevents stack overflow by defining every call separately
let SEED: bigint | null = null;

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// -----------------------------------------------------------------------------
// 2. TYPE DEFINITIONS & UTILITIES
// -----------------------------------------------------------------------------

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

/**
 * Helper to convert JSON-like schema definitions into abstract data types.
 */
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(val => typeof val === "string" || (val && String(val).toLowerCase() !== "")) // Handle empty strings or nulls safely if needed later; here we assume valid types for this demo.
}

/**
 * Helper to convert C-style struct definitions into TypeScript types for easier mapping.
 */
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract DataType Class for validation and metadata.
 */
export class AbstractDataType<T = AlchemyDatabaseType> {
  private readonly _metadata: Record<string, string>; // Schema keys -> type mappings

  constructor(metadata?: Partial<Record<string, string>>) {
    this._metadata = Object.assign({}, metadata);
  }

  /**
   * Validate that the schema matches expected types based on key.
   */
  validate(key: string): boolean {
    return (this._metadata as any)[key] === "string" ||
      ((typeof this._metadata[key]) as any) === "integer"; // C/C# style integer is typically int64/uint64 in Rust, mapped to number here.
  }

  /**
   * Convert a type identifier (e.g., 'integer') into the actual value representation for use with database generators.
   */
  convertToValue(key: string): T {
    return this._metadata[key] as T; // Returns native primitive or boolean if not specified, otherwise inferred from context.
  }

  /**
   * Convert a type identifier (e.g., 'string') into the actual value representation for use with database generators.
   */
  convertToValue(key: string): string {
    return this._metadata[key] as string; // Returns native primitive or boolean if not specified, otherwise inferred from context.
  }

  /**
   * Generate a random integer within bounds [0, MAX_DEPTH - 1].
   */
  generateRandomInt(): number {
    const seed = SEED ?? Math.random() * BigInt(MAX_DEPTH); // Seed for randomness; if null, uses current time's timestamp as fallback.
    
    return crypto.getRandomValues(new Uint8Array(4))[0] | (seed % MAX_DEPTH) / MAX_DEPTH;
  }

  /**
   * Generate a random number within [1, MAX_DEPTH].
   */
  generateRandomIntInRange(): number {
    const rng = crypto.randomRange(Math.floor(MAX_DEPTH), Math.ceil(MAX_DEPTH)); // Use range to avoid duplicates if needed.
    return rng % MAX_DEPTH;
  }

}

// -----------------------------------------------------------------------------
// 3. CORE MAPPING LOGIC (SCHEMA TO TYPES)
// -----------------------------------------------------------------------------

/**
 * Core mapping logic: convert standard C/C# struct field names into a generic "Schema" key that can be mapped to concrete types via the helper function returning string, integer, or null.
 */
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Reverse mapping: convert a type identifier into the actual value representation for use with database generators.
 */
export function getTypeToValue(typeId:
