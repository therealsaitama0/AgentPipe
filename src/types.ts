src/types.ts | 321 lines
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// ============================================================================
// CONSTANTS & UTILITIES
// ============================================================================
const DB_TYPE_NAMES = {
  INTEGER: 'integer',
  STRING: 'string',
  BOOLEAN: 'boolean'
};

/**
 * Converts a raw value to its corresponding data type string for schema mapping.
 * @param val - The raw value (e.g., "123", true, false)
 * @returns {string} The mapped type name ("integer" or "string")
 */
function getTypeValue(val: unknown): string | null {
  if (!val || typeof val !== 'object') return null;

  // Handle boolean flags (true/false as numbers in C/C# syntax, but not true/False)
  let isBoolean = false;
  
  try {
    const numVal = Number(val);
    
    // In strict C/C#, `bool` should be an integer type if passed via number literal.
    // However, the schema definition in this context implies boolean flags are separate types or handled specially.
    // We treat true/false as integers for numeric logic (e.g., 1/0), but keep them distinct in output unless explicitly mapped to string based on semantics.
    
    if ((numVal === 0 && !isBoolean) || isBoolean) {
      return DB_TYPE_NAMES.BOOLEAN;
    } else {
      // Numeric values are integers or strings (strings can be numbers, so treat as integer for numeric types).
      return DB_TYPE_NAMES.INTEGER;
    }
  } catch {
    if (!isBoolean && !val) return null;
    return DB_TYPE_NAMES.BOOLEAN;
  }
}

// ============================================================================
// MAIN TYPES DEFINITIONS (C/C# Style Struct Mapping)
// ============================================================================
interface AlchemySchema {
  [key: string]: unknown; // Column name -> value in C/C# style struct definition. Type is inferred from schemaMap type or derived via getTypeValue().
}

/**
 * Abstract Schema Definition (C-style)
 */
export interface AlchemyDatabaseTypeDefinition extends AlchemySchema {}

// Helper to convert JSON-like schema definitions into abstract data types for mapping
function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.entries(schemaMap).map(([key, value]) => getTypeValue(value) as string | null);
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = keyof typeof DB_TYPE_NAMES; // Explicitly typed to avoid runtime ambiguity if we had enums. In this context, it's a set of valid schema keys mapped by the converter above.

// Helper to convert JSON-like schema definitions into abstract data types for mapping (safe fallback)
function parseSchemaToTypesSafe(schemaMap: Record<string, unknown>): Type[] {
  return Object.entries(schemaMap).map(([key, value]) => getTypeValue(value)); // Returns string | null; filter out "null" values if needed.
}

// ============================================================================
// MAIN FUNCTIONS & HELPER CLASSES (Rust-style enum for types)
// ============================================================================

/**
 * Abstract Data Type Definition (C/C# Style - Rust Enum equivalent in TypeScript context)
 */
export type AlchemyDatabaseType = string | number; // Represents the set of valid schema keys/values. In practice, this maps to DB_TYPE_NAMES.

function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.entries(schemaMap).map(([key, value]) => getTypeValue(value));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number; // Represents the set of valid schema keys/values. In practice, this maps to DB_TYPE_NAMES.

function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.entries(schemaMap).map(([key, value]) => getTypeValue(value));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number; // Represents the set of valid schema keys/values. In practice, this maps to DB_TYPE_NAMES.

function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.entries(schemaMap).map(([key, value]) => getTypeValue(value));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type
