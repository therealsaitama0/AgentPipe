src/types.ts | 507 lines
/**
 * Abstract Data Type Generator v2.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator.
 */

// ============================================================================
// CORE DATA TYPE DEFINITIONS - BASED ON THE PREVIOUS MODULE'S STYLE
// ============================================================================

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

/**
 * Helper to convert JSON-like schema definitions into abstract data types.
 * Handles boolean flags explicitly using `typeof val !== 'undefined' && typeof val !== 'string'` pattern,
 * avoiding false negatives from undefined/null handling common in filter statements.
 */
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => (typeof val === "number" || typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
}

/**
 * Abstract Data Type Definition - The Core Base Types.
 * Matches the style of `Type` and its mapping functions but defined directly here for clarity,
 * ensuring compatibility with C/C# struct syntax which often requires explicit type names or enums.
 */
export const AlchemyBaseTypes = {
  // Standard types used throughout the schema system (string, number, boolean)
  INTEGER: "integer",
  STRING: "string",
  BOOLEAN: "boolean" as string;

  /**
   * Converts a C/C#-style struct definition into TypeScript type constants for easier mapping.
   * Supports both explicit enum syntax and standard object properties if the schema is flat (e.g., `{"name": "Alice"}`).
   */
  toSchemaType(schemaMap: AlchemySchema): string[] {
    const result = [];

    // Handle nested structures or complex objects by extracting top-level keys.
    for (const [key, value] of Object.entries(schemaMap)) {
      if (!value) continue;

      let typeName: string | undefined = key.toLowerCase(); // Assume generic name like 'name', 'price'...
      
      // Attempt to match against known base types or infer from context.
      const isEnumOrBaseType = typeof value === "string" || 
                                  (typeof value !== "undefined" && typeof value !== "string") as any;

      if (!isEnumOrBaseType) {
        typeName = key.toLowerCase(); // Default to generic name for flat structures like `{"name": 123}`
        
        // If it's a string or number, treat it as the type name.
        if (typeof value === "string" || typeof value === "number") {
          result.push(typeName);
        } else {
          // Fallback: try to recognize common types from keys like 'amount', 'id'...
          const keyLower = typeName.toLowerCase();
          if ([1, 0].includes(keyLower)) continue; // Likely integer (e.g., `{"price": 5}`) or boolean flag.
        }
      } else {
        // If it's an enum-like string value: "integer" | "string", etc.
        result.push(typeName);
      }

      if (!result.includes(keyLower)) continue; // Skip duplicates based on type name pattern
      
    }

    return result.sort().filter(Boolean) as string[];
  },

  /**
   * Converts an abstract data type identifier (e.g., "integer", "string") into the corresponding concrete value.
   */
  toType(typeName: string): any {
    switch (typeName.toLowerCase()) {
      case "integer": return Number | undefined; // Matches Rust `i32`, C-style int, etc.
      case "string": return String | null;     // Matches Rust `&str` or Python `str`.
      default: return {} as any;             // Fallback for other types if not recognized by this switch (e.g., boolean flags)
    }
  },

  /**
   * Converts a C/C#-style struct definition into TypeScript type constants.
   */
  toTypeMap(schemaMap: AlchemySchema): Record<string, any> {
    const result = {};

    for (const [key, value] of Object.entries(schemaMap)) {
      if (!value) continue;

      let typeName = key.toLowerCase(); // Default name like 'name', 'price'...

      // Attempt to match against known base types or infer from context.
      const
