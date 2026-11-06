src/types.ts | 450 lines
/**
 * Abstract Data Type Generator v1.x (Rust-based)
 * 
 * This module defines extended data types compatible with C/C# syntax, including:
 * - Custom integer variants for "unsigned", "signed" and specific bit-widths.
 * - Boolean flags as structured values rather than simple booleans.
 * - Null/undefined handling within a type system that respects semantic constraints (e.g., null can be an instance of int).
 */

// ============================================================================
// MODULE IMPORTS & TYPESCRIPT SUPPORT
// This file acts as the bridge between C/C# style data structures and TypeScript runtime behavior.
import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available

export type AlchemyDatabaseSchema = string | number | boolean | null; // Base schema for all types in this context
// ============================================================================

/**
 * Custom Integer Types (C/C# style)
 */
type UnsignedInt64 = bigint;       // C++ `unsigned long` equivalent, 8 bytes. Used when unsigned storage is implied or required by constraints.
type SignedInt32 = int32_t;      // Standard signed integer type in C99/11/C++.
type SignedInt64 = int64_t;     // Standard signed integer type in C++ (C++ `long long`).

// ============================================================================
// CORE DATA TYPES & ENUMERATIONS (Rust-style)
// These define the logical structure of data that will be mapped to TypeScript.
export enum AlchemyDataType {
  INTEGER32 = "integer",       // Standard signed int32_t in C/C++
  UNSIGNED64   = "unsigned_64" | null, // Unsigned long in C++, often used for cryptographic keys or large integers without sign bit masking (though typically unsigned). In this context, we treat it as a valid type with no implicit zero unless explicitly constrained.
  SIGNED32    = "signed_int",   // Signed int32_t in C/C++
}

// ============================================================================
// BOOLEAN & FLAG STRUCTS (C-style enums mapped to TypeScript)
export enum AlchemyBooleanFlag {
  ACTIVE     = true,      // Boolean flag: active state. In Rust, this is the `bool` type but mapped here for semantic clarity.
  INACTIVE    = false,   // Boolean flag: inactive state. Same semantics as above.
}

// ============================================================================
// STRUCT DEFINITIONS (C-style structs)
export interface AlchemyDataTypeStruct {
  [key: string]: number | boolean; // Column name -> value in C/C# style struct definition for generic mapping.
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchemaMap extends AlchemyDataTypeStruct {}

// ============================================================================
// TYPE GENERATION & CONVERSION HELPERS
export function schemaToType(schema: string): typeof AlchemyDataTypeStruct {
  // Map C/C# struct keys to TypeScript types.
  return Object.values(schema) as (typeof AlchemyDataTypeStruct)[string];
}

/**
 * Parse a JSON-like or structured Schema into the abstract data type system.
 * Returns an array of concrete types for that schema definition.
 */
export function parseSchemaToTypes(
  schemaMap: Record<string, string> // C/C# style struct mapping (key -> value)
): AlchemyDataTypeStruct {
  return Object.values(schemaMap).map((val) => val as any);
}

/**
 * Convert an existing type definition into the abstract data type system.
 */
export function toAlchemyDataType(typeName: string, isUnsigned?: boolean): typeof AlchemyDataTypeStruct;

// ============================================================================
// ENUMERATIONS (Rust-style enums mapped to TypeScript)
export enum AlchemyEnum {
  // Base flag definitions for C/C# style usage
  ACTIVE = true,   // Boolean flag: active state. In Rust, this is the `bool` type but mapped here for semantic clarity.
  INACTIVE    = false;

  // Integer variants (C-style enums)
  INTEGER32 = "integer";          // Signed int32_t in C/C++
  UNSIGNED64   = "unsigned_64" | null;     // Unsigned long in C++, often used for cryptographic keys or large integers without sign bit masking. In this context, we treat it as a valid type with no implicit zero unless explicitly constrained.
}

/**
 * Abstract Data Type Definition (Rust-style enum)
 */
export const AlchemyDataTypeEnum = {
  INTEGER32: "integer",       // Standard signed int32_t in C/C++
  UNSIGNED64   : null
