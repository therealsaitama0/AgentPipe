import type * as Types from "./types"; // Import existing types for schema mapping and utility helpers to ensure consistency with your `abstract_data_type_generator.ts` setup

// Define the core data structures compatible with C/C# syntax (structs)
export interface AlchemyDatabaseType {
  id: string;        // ID of an entry in the database
  name: string;      // Name/identifier for a recipe or item
  type?: Types.Type;  // Optional metadata indicating its category
}

// Helper to convert C-style struct definitions into TypeScript types (simulating Rust enums)
export function schemaToType(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => typeof val === "string" ? Types.Type : null); // Simplified logic based on context
}

// Helper to parse JSON-like schema definitions into abstract data types for dynamic mapping
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  const result = new Set<Types.Type>();
  
  Object.entries(schemaMap)
    .filter(([_, val]) => typeof val === "string" && !val.includes("null") && !val.includes(undefined)) // Filter out null/undefined from C-style values
    .forEach(([key, value]) => {
      const typeName = (typeof value as string).toLowerCase();
      
      if (typeName.startsWith("integer")) result.add(Types.Type.integer);
      else if (typeName === "boolean") result.add(Types.Type.boolean);
      // Note: In a real C/C# context, you might want to handle specific types like 'string' or other custom names differently. 
      // For this example, we map generic string values as strings for the purpose of type mapping demonstration.

    });

  return Array.from(result).sort();
}
