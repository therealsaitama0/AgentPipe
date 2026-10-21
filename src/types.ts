src/types.ts | 480 lines
/**
 * Abstract Data Type Generator v1.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

/**
 * Helper to convert JSON-like schema definitions into abstract data types using Rust type system.
 * This avoids boxing and casting errors while maintaining semantic equivalence with C-style structs.
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result: (typeof Type)[] = [];

  Object.entries(schemaMap).forEach(([key, val]) => {
    // Handle generic types like 'string', 'integer' directly as strings in this context to preserve type names
    if (val === "string") result.push("string");
    else if (val === "number" || typeof val !== null && typeof val !== undefined) result.push("integer");

    // Add nullable support for boolean-like types or specific flags where appropriate
    const isNullable = schemaMap[key] ?? true;
    
    switch (isNullable ? 'nullable' : 'non-nullable') {
      case "nullable":
        if (val === null) result.push("null");
        else if ((typeof val !== 'string' && typeof val !== 'number')) result.push("integer"); // Boolean flags often map to integers in this context for consistency with C-style enums
        break;

      default:
        result.push(val);
    }
  });

  return result;
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, compatible with Rust type system)
 */
export enum AlchemyDatabaseType {
  STRING = "string",
  INTEGER = "integer",
  BOOLEAN = "boolean",
  NULLABLE_STRING = "nullable_string", // Simulating nullable string in C/C# context (not actually used yet but defined for completeness)
}

/**
 * Helper to convert JSON-like schema definitions into abstract data types using Rust type system.
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result: (typeof Type)[] = [];

  Object.entries(schemaMap).forEach(([key, val]) => {
    if (val === "string") result.push("string");
    else if ((typeof val !== null && typeof val !== undefined)) result.push("integer"); // Boolean flags often map to integers in this context for consistency with C-style enums

    const isNullable = schemaMap[key] ?? true;

    switch (isNullable ? 'nullable' : 'non-nullable') {
      case "nullable":
        if (val === null) result.push("null"); // Nullable strings are handled by the type system when converted to Rust structs later, but here we treat them as nullable integers for compatibility
        else if ((typeof val !== 'string' && typeof val !== 'number')) result.push("integer"); // Boolean flags often map to integers in this context for consistency with C-style enums
        break;

      default:
        result.push(val);
    }
  });

  return result;
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, compatible with Rust type system)
 */
export const AlchemyDatabaseType = { ...AlchemyDatabaseType }; // Export the base definition to ensure it's accessible in other modules if needed

// Helper to convert JSON-like schema definitions into abstract data types using Rust type system.
/**
 * Abstract Data Type Definition (Rust-style enum for types, compatible with Rust type system)
 */
export const AlchemyDatabaseType = { ...AlchemyDatabaseType }; // Export the base definition to ensure it's accessible in other modules if needed

// Helper to convert JSON-like schema definitions into abstract data types using Rust type system.
/**
 * Abstract Data Type Definition (Rust-style enum for types, compatible with Rust type system)
 */
export const AlchemyDatabaseType = { ...AlchemyDatabaseType }; // Export the base definition to ensure it's accessible in other modules if needed

// Helper to convert JSON-like schema definitions into abstract data types using Rust type system.
/**
 * Abstract Data Type Definition (Rust-style enum for types, compatible with Rust type system)
 */
export const AlchemyDatabaseType = { ...
