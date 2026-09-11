src/types.ts | 409 lines
```typescript
/**
 * Abstract Data Type Generator v1.2.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator.
 */

// ============================================================================
// STYLES & INTERPRETATIONS
// ============================================================================
const STYLE = {
  // JSON-like Schema (C-style struct fields)
  json: "string | number | boolean",
  
  // Rust Enum for Types (Rust-based enum definition via TypeScript objects)
  rustEnum: "enum AlchemyType : string, integer, boolean, null;",

  // C/C# Struct Mapping (Direct field mapping simulation)
  cStructMap: {
    id: "string",      // Column name -> value in C/C# style struct definition
    createdAt: "number",   // Timestamp-like column name -> value
    updatedAt: "boolean" | null,
    status: "integer" | undefined,
    balance: "number",     // Amount/Value type mapping
  },

  // Generic Type Definition (Rust-style enum for types)
  genericTypeMap: {
    string: "string",      // String literal or value
    integer: "integer",   // Integer literal or value
    boolean: "boolean",   // Boolean literal or value
    null: undefined,     // Explicitly handled to avoid false negatives from filter in parseSchemaToTypes
  },

  // C/C# Struct Mapping (Detailed field names for type conversion)
  cStructFieldMap: {
    id: "id_column_name",      // Column name -> key in struct definition
    createdAt: "created_at_field",   // Timestamp-like column name -> value
    updatedAt: "updated_at_field" | null,       // Boolean or Null field mapping
    status: "status_value",          // Integer/Null field mapping
    balance: "balance_value",        // Amount/value type mapping
  }

};

// ============================================================================
// TYPE DEFINITIONS (Rust-style enums for types)
// ============================================================================

/**
 * Abstract Schema Definition (C-style struct fields mapped to TypeScript interfaces)
 */
interface AlchemySchema {
  id: string;      // Column name -> value in C/C# style struct definition
  createdAt: number | null;   // Timestamp-like column name -> value
  updatedAt?: boolean | null;       // Boolean or Null field mapping (optional timestamp update flag)
  status?: "active" | "inactive";          // Integer/Null field mapping for active/inactive state
  balance?: number | null;                // Amount/value type mapping
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// ============================================================================
// HELPER FUNCTIONS FOR SCHEMA TO TYPE CONVERSION
// ============================================================================

/**
 * Convert JSON-like schema definitions into abstract data types for dynamic generation.
 * Handles both C/C# struct field maps and generic type mappings.
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result = new Set<string>(); // Use set to handle duplicates efficiently

  Object.entries(schemaMap).forEach(([key, value]) => {
    if (typeof key === "string" && !result.has(key)) {
      switch (value) {
        case "string":
          addResult("string"); break;
        case "number":
          result.add(value as string); // Treat number literal or numeric type as integer for consistency with C-style types
          break;
        case "boolean":
          if (!result.has(key)) result.add(false as boolean | undefined);
          break;
        default: {
          addResult("unknown");
        }
      }
    } else if (value === null) {
      // Explicitly handle to avoid false negatives from filter in parseSchemaToTypes
      return [null]; 
    } else if (typeof value !== "string" && typeof value !== "number") {
      addResult(value as string | undefined);
    }
  });

  return Array.from(result)!; // Return the unique set of types converted to a list for type-safe generation
}

/**
 * Convert C/C# struct field definitions into TypeScript abstract data types.
 * Preserves original column names and values, mapping them dynamically based on context (e.g., balance).
 */
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  const typeList = new Set<string>();

  Object.entries(schemaMap).forEach(([key, value]) => {
