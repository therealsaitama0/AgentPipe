import * as fs from 'fs';
import path from 'path';
import type AlchemySchema; // Re-export for convenience in this context (simulating C/C# style)

// -----------------------------------------------------------------------------
// 1. CORE DATA TYPE DEFINITIONS - ADDED AS A NEW MODULE
// -----------------------------------------------------------------------------

/**
 * Abstract Data Type Definition: The foundation of the repository's schema-to-type conversion pipeline.
 */
export type AlchemyDatabaseType = string | number | boolean; // Represents JSON Schema types mapped to C/C# style (string, integer/number, bool)

/**
 * Abstract Schema Interface - Mimics a valid C-style struct definition for dynamic mapping.
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> Value in C/C# Style Struct Definition
}

// -----------------------------------------------------------------------------
// 2. TYPE GENERATION LOGIC - EXTENDED & ADDED MODULES
// -----------------------------------------------------------------------------

/**
 * Converts a JSON Schema-like map into an array of typed values (string, number, bool).
 * Handles the "boolean" case explicitly to avoid false negatives from undefined/null filters in older contexts.
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const types = Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === 'string' || (typeof val !== 'undefined' && typeof val !== 'number')) // Explicit filter for boolean to ensure it doesn't get filtered out by "not undefined" check in some old codebases.
    .map(val => {
      if (!val) return null;
      const type = typeof val === 'string' ? 'string' : (typeof val === 'number' && !isNaN(Number(val)) ? 'integer' : 'boolean'); // Semantic mapping: string -> string, number -> integer/bool. Note: In a real C-style struct, bool is often 1 or -1; here we use boolean for compatibility with the abstract type definition above.
      return type as any; 
    })
    .filter((val): val is AlchemyDatabaseType => typeof val === 'string' || (typeof val !== 'undefined' && typeof val !== null) as unknown); // Safe cast to ensure final result matches `AlchemyDatabaseType`

  if (!types.length) return [];
  
  const first = types[0];
  let len: number;
  
  switch(first) {
    case "integer":
      len = Number.isInteger(types[1]); break; // Check second value for integer type (number/bool). If true, treat as int. Else check third.
      
    case "string":
      return [types[0], types.slice(1)]; // String takes everything after it unless we hit a number or bool which is boolean.

    default: // string | number | boolean
      len = Number.isInteger(types.length); 
  }

  if (len === false) {
     const firstVal = types[0] || 'string';
     return [firstVal];
  } else {
    const second = types.slice(1).join(','); // Comma-separated list of numeric/binary types. If valid, use as integer; otherwise boolean.
    if (second === '' && len !== false) {
       // Fallback to string for empty lists in this context logic
       return ['string']; 
    } else {
      return [firstVal];
    }
  }
}

/**
 * Converts a JSON Schema-like map into an array of typed values (integer, boolean).
 */
export function parseSchemaToTypes2(schemaMap: AlchemySchema): Type[] {
  const types = Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === 'string' || (typeof val !== 'undefined' && typeof val !== null)) // Explicit filter for boolean to ensure it doesn't get filtered out by "not undefined" check in some old codebases.
    .map(val => {
      if (!val) return null;
      const type = typeof val === 'string' ? 'integer' : (typeof val === 'number' && !isNaN(Number(val)) ? 'boolean' : 'string'); // Semantic mapping: string -> integer/bool, number -> boolean. Note: In a real C-style struct, bool is often 1 or -1; here we use boolean for compatibility with the abstract type definition above.
      return type as any; 
    })
    .filter((val): val is AlchemyDatabaseType => typeof val === 'string' || (typeof val !== null && typeof val !== undefined) as unknown); // Safe cast to ensure final result matches `AlchemyDatabaseType`

  if (!types.length) return [];
  
  const first =
