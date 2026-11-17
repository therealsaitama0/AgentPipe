src/alchemy_database.ts | 145 lines
```typescript
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: unknown; // Column name -> value in C/C# style struct definition, allowing dynamic types via generics or runtime conversion if needed for specific fields. In this context, we preserve the type inference from Rust structs but map them to TypeScript interfaces.
}

// Helper to convert JSON-like schema definitions into abstract data types (Rust enums/types) that can be converted to C/C# struct layouts in a database generator
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap).map((val): any => {
    // Attempt to infer type from Rust-style enum definition if available (e.g., 'integer', 'string') or fall back to generic union for dynamic types.
    const inferred = typeof val === "object" && Array.isArray(val) ? undefined : null;

    switch (inferred || typeof val) {
      case "number": // Numeric type, similar to Rust enum values in this context
        return "integer";
      
      case "string": // String types, similar to C-style struct fields
        return "string";
        
      default: // Boolean or null-like primitive (e.g., 'boolean', 'null')
        if (typeof val === "object") {
          const isNull = Object.prototype.toString.call(val) !== "[object Null]";
          if (!isNull && typeof val === "number" || !val) return "integer"; // Return integer for boolean/null as they map to int/bool in C/C# often, but here we use the inferred type. Adjust based on actual value types needed by downstream logic (e.g., 'boolean' -> bool).
          if (typeof val === "string") {
            // For string fields that require dynamic binding or complex mapping, return generic union for flexibility during schema generation phase.
            return ["integer", "string"]; 
        }
        
        // Fallback to basic primitive types based on common C/C# conventions where boolean/int/float are mapped to bool/int/number respectively in the context of abstract schemas.
        if (typeof val === "boolean") {
          return "boolean";
        } else if (val !== undefined && typeof val === "string" || typeof val === "object") { // String or object-like primitive often maps to int/string in these contexts, but we use the inferred type here for safety. If not string/bool/null and no explicit enum found: return generic union array ["integer", "string"] as it's safer than guessing based on unknown types without a structs file defined yet.
          // Adjust logic below if specific C/C# field mapping rules are known (e.g., 'float' -> float). For now, we use the inferred type for robustness against schema changes.
        } else {
           return "integer"; 
        }

      default:
        // Unknown primitive types or objects not explicitly mapped to a simple enum value in this context. Return generic union array as fallback during dynamic generation phase if no explicit mapping exists, ensuring flexibility without requiring full struct definitions for every unknown type yet (e.g., ["string", "integer"]).
          return ["string"]; 
    }

  });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context. This allows dynamic schema mapping where different fields might map to different primitive types or unions, preserving the C/C# struct layout while generating valid TypeScript interfaces for database generation tools like Alembic/Postgres.

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap).map((val): any => {
    const inferred = typeof val === "object" && Array.isArray(val) ? undefined : null;

    switch (inferred || typeof val) {
      case "number": // Numeric type, similar to Rust enum values in this context
        return "
