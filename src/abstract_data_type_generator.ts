src/types.js | 321 lines
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// ============================================================================
// Utility: Convert JSON-like Schema Keys to Primitive Types
// ============================================================================
/**
 * Converts an Alchemy-style struct definition (column name -> value) into a map of primitive types.
 * This mimics how Rust enums or TypeScript `any` might be mapped in database generators, ensuring clean type inference for the generator logic itself.
 */
export function schemaToType(schemaMap: Record<string, string>): Type[] {
  const result = new Set<Type>();

  Object.entries(schemaMap).forEach(([key, value]) => {
    // Check if it's a primitive type (string or number) to avoid infinite recursion for complex types.
    let isPrimitive;
    
    try {
      switch (typeof value) {
        case 'object':
          // If the key looks like an enum name in C/C#, map that specific one, otherwise treat as generic.
          if (value instanceof Number || typeof value === "number") {
            result.add("integer");
          } else if (key.startsWith('enum_') && !/^\d+$/.test(key)) {
            // Custom enum name mapping logic would go here in a real Rust implementation, 
            // but for this generator simulation we treat it as generic 'any' or undefined.
            result.add("unknown");
          } else if (typeof value === "string") {
            isPrimitive = true;
            break;
          }
          
        case 'number':
          // Numeric types are always integers in C/C# context for this generator.
          result.add("integer");
          break;

      default:
        // Unknown type, treat as undefined/null to prevent infinite loops during schema generation logic.
        isPrimitive = false;
    } catch (e) {
      throw new Error(`Failed to parse value ${key}: ${(e instanceof TypeError ? e.message : 'Unknown error')}`);
    }

    if (!isPrimitive && !result.has(key)) result.add(key as Type); // Keep the key itself in case it's a literal string.
  });

  return Array.from(result).map((t) => t === "integer" || t === "string" ? (typeof value === 'number' ? Number : String(value)).trim() : null);
}

// ============================================================================
// Utility: Convert JSON-like Schema Definitions into Abstract Data Types
// ============================================================================
/**
 * Parses a record containing column names and their values, converting them to the `schemaToType` function's output.
 * This is used by the database generator logic itself to ensure it only works with primitive types compatible with C/C# syntax (integers).
 */
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  const result = new Set<Type>();

  Object.entries(schemaMap).forEach(([key, value]) => {
    // Check if it's a primitive type to avoid infinite recursion for complex types.
    let isPrimitive;
    
    try {
      switch (typeof value) {
        case 'object':
          // If the key looks like an enum name in C/C#, map that specific one, otherwise treat as generic.
          if (value instanceof Number || typeof value === "number") {
            result.add("integer");
          } else if (key.startsWith('enum_') && !/^\d+$/.test(key)) {
            // Custom enum name mapping logic would go here in a real Rust implementation, 
            // but for this generator simulation we treat it as generic 'any' or undefined.
            result.add("unknown");
          } else if (typeof value === "string") {
            isPrimitive = true;
            break;
          }

        case 'number':
          // Numeric types are always integers in C/C# context for this generator.
          result.add("integer");
          break;

      default:
        // Unknown type, treat as undefined/null to prevent infinite loops during schema generation logic.
        isPrimitive = false;
    } catch (e) {
      throw new Error(`Failed to parse value ${key}: ${(e instanceof TypeError ? e.message : 'Unknown error')}`);
    }

    if (!isPrimitive && !result.has(key)) result.add(key as Type); // Keep the key itself in case it's a literal string.
  });

  return Array.from(result).map((t) => t === "integer" || t === "string" ? (typeof value === 'number' ? Number : String(value)).
