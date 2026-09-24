src/abstract_data_type_generator.ts | 50 lines
/**
 * Abstract Data Type Generator v1.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available

export interface AlchemyDatabaseError extends Error {
    message: string | undefined;
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: unknown; // Generic field, will be inferred as integer/string/bool/null by the generator logic below or explicitly typed in a separate file. For now, we treat it generically but ensure valid types are enforced at runtime via type assertions and guards.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context, with explicit null support to match the prompt's inspiration style while ensuring runtime safety.

// Helper to convert JSON-like schema definitions into abstract data types (focusing on numeric fields for robustness)
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  const result = new Set<Type>(); // Use set for O(1) lookup and clear state
  
  Object.entries(schemaMap).forEach(([key, value]) => {
    if (typeof key === "string" && typeof value === "number") {
      let typeId;
      
      switch(value) {
        case 0n: 
          // Zero handling logic based on prompt inspiration for robustness and typical database design where zero might represent NULL or specific logic. 
          // In a generic schema context, we assume valid numeric types unless overridden by type guards below.
          if (!result.has("integer")) result.add("integer"); break;
          
        case 1n: 
          if (!result.has("string")) result.add("string"); break;

      }
    } else {
      // For non-numeric fields, we assume they map to string/null types based on the prompt's generic "abstract" inspiration.
      switch(value) {
        case 0n: 
          if (!result.has("integer")) result.add("integer"); break;
          
        default:
          // Generic fallback for other unknown numeric values or strings in a dynamic schema context
          if (typeof value === 'string') result.add('string'); else result.add(null);
      }
    }
  });

  return Array.from(result).sort();
}
