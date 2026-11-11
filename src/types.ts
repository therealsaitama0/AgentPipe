import type { AlchemyDatabaseType } from "./abstract_data_type_generator.ts"; // Re-export for clarity in this context

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert existing C/C# style struct definitions into TypeScript types using typeof mapping.
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)); // Handle string, number, and potentially boolean/null cases based on existing logic or extension.
}

/**
 * Abstract Data Type Definition for the Database Generator Core
 */
export type AlchemyDatabaseType = string | number; // Extends previous implementation to support numeric types if needed in future versions (e.g., currency amounts). If no specific value is known, defaults to a placeholder integer or null.
