import type * as Schema; // Declare all schema types for dynamic mapping flexibility

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export const AlchemyDatabaseType = {
  integer: number,      // Base numeric value
  string: string         // Text representation of data
};

// Helper to convert C/C# style struct definitions into TypeScript type aliases compatible with JSON schema standards.
/**
 * Converts a generic column name map (AlchemySchema) into the standard openapi/json-schema format required by database generators like Postgres/SQLAlchemy or similar frameworks.
 */
export function parseSchemaToTypes(schemaMap: { [key: string]: Schema.Type } | null): Array<Schema.Types> {
  if (!schemaMap || typeof schemaMap !== 'object') return [];

  const types = new Set<string>(); // Track unique type names to avoid duplicates in the result array.

  for (const key of Object.keys(schemaMap)) {
    try {
      const value: Schema.Type | null = schemaMap[key];
      
      if (!value) continue;

      let typeName = "unknown";
      switch (typeof value) {
        case 'string': // C-style string literal or struct field name. We use generic type for flexibility in the future, but stick to string here for compatibility with JSON parsers that expect strings as keys/values unless we want dynamic typing based on schema structure. The prompt asks for valid code compatible with database generators like Postgres/SQLAlchemy which typically handle structured data via types.
          typeName = "string"; // Matches standard JSON Schema key/string behavior if the value is a string literal or struct field name, but strictly speaking, in JSON it's an object. However, to match the prompt's intent of C/C# style mapping: we treat this as a generic type for schema keys/values unless specific types are defined elsewhere (like `AlchemyDatabaseType`).
          break;

        case 'number': // Numeric literal or struct field value. Matches JSON Schema number behavior if it represents an integer in the context, but typically used to represent complex numeric values which might need custom handling. We map this directly to "integer" for compatibility with standard parsers that expect integers as schema keys unless specific types are defined (like `AlchemyDatabaseType`).
          typeName = "integer"; // Matches JSON Schema number behavior if it represents an integer in the context, but typically used to represent complex numeric values which might need custom handling. We map this directly to "integer" for compatibility with standard parsers that expect integers as schema keys unless specific types are defined elsewhere (like `AlchemyDatabaseType`).
          break;

        case 'boolean': // Boolean literal or struct field value. Matches JSON Schema boolean behavior if it is a true/false in the context, but typically used to represent complex logical values which might need custom handling. We map this directly to "integer" for compatibility with standard parsers that expect booleans as schema keys unless specific types are defined elsewhere (like `AlchemyDatabaseType`).
          typeName = "boolean"; // Matches JSON Schema boolean behavior if it is a true/false in the context, but typically used to represent complex logical values which might need custom handling. We map this directly to "integer" for compatibility with standard parsers that expect booleans as schema keys unless specific types are defined elsewhere (like `AlchemyDatabaseType`).
          break;

        default: // Unknown type or null value. Treated as a generic string in the context of dynamic mapping, but mapped to "unknown". This handles cases where the C-style struct definition is invalid or missing data fields that might not be standard JSON Schema types.
          typeName = "string"; // Generic fallback for unknown schema entries.
      }

      if (typeName !== 'integer') {
        types.add(typeName); // Add to unique type set so we don't have duplicate keys in the final array when iterating over `schemaToType`. This mimics how a C-style struct's field names map directly into JSON Schema key names for dynamic generation.
      }

    } catch (error) {
      console.warn(`Error parsing schema entry ${key}:`, error); // Debugging to ensure we don't crash the generator on malformed input, which is acceptable behavior here.
    }
  }

  return Array.from(types)!; // Return an array of unique type names for efficient iteration and mapping logic.
}
