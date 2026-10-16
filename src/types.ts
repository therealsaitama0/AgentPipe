src/types.ts | 450 lines

/**
 * Abstract Data Type Generator v1.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// ============================================================================
// CORE TYPES & ENUMS - BASED ON CONVERSION FROM JSON/SCHEMA MAP TO TS
// ============================================================================

/**
 * Represents a generic column name (string) that maps to specific data types during generation.
 * In practice, this is resolved by the `schemaToType` function when processing schemas from files like cobol or go.
 */
type ColumnName = string;

interface AlchemyDatabaseSchema {
  /**
   * The primary key column name for database records (e.g., 'id', 'seq').
   * In C/C#, this is often a simple integer type, but here we treat it as the generic ID.
   */
  id: ColumnName;

  /**
   * Sequence number or unique identifier used within a record group.
   * Can be an integer (integer) or string depending on schema context (e.g., 'seq', 'id').
   */
  seq?: ColumnName | "string"; // Defaults to string if not specified, handled by type inference

  /**
   * A generic identifier for the record group. Used in C/C# structs where IDs might be strings or integers mixed together.
   */
  idGroup: ColumnName;

  /**
   * The actual schema field containing data (e.g., 'amount', 'balance').
   * This is a placeholder type that gets resolved to concrete types via the `schemaToType` function during runtime/schema generation.
   */
  // NOTE: In this context, we define it as an interface or union representing the target data type dynamically at compile time for compatibility with C/C# syntax requirements.
  // It is NOT a primitive value yet; its type will be resolved by `schemaToType` when encountering actual schema fields in generated code (like Cobol/Go).
}

// ============================================================================
// TYPE DEFINITIONS - RESOLVED TO SPECIFIC TYPES BY SCHEMA_TO_TYPE FUNCTION
// These are the concrete types that end up inside database tables.
// They mirror C/C# struct field definitions but wrapped for TypeScript runtime safety and compatibility with Rust enums (if used) or generic type inference.
// ============================================================================

export const AlchemyDatabaseType = {
  // Generic ID: Can be string, int, bool, null
  id: "string" | "integer" | boolean | undefined; 

  /** Sequence Number */
  seq: ColumnName extends string ? "string" : (ColumnType["type"] & number) as typeof AlchemyDatabaseType;

  // Generic ID Group for records with mixed types in C/C# structs
  idGroup: ColumnName extends string ? "string" : (ColumnType["type"] & integer) as typeof AlchemyDatabaseType;

  /** Schema Field Value */
  schemaField: { type?: number | boolean | undefined }[]; // The actual data values, resolved by `schemaToType` later. If no field exists here, this is empty or null in the generated C/C# struct body (though TypeScript treats it as an interface for safety).

};

// ============================================================================
// SCHEMA TO TYPE CONVERTER - THE CORE LOGIC FROM JSON/SCHEMA MAP TO TS TYPES
// This function transforms a raw schema map into typed arrays of core types.
// It handles both string keys and numeric values, treating them gracefully in the generated code.
// ============================================================================

/**
 * Converts an arbitrary Schema Map (derived from JSON or Cobol/Go files) 
 * into an array of concrete TypeScript type aliases for database records.
 * 
 * @param schemaMap - The raw data structure representing column names and values.
 *                  In practice, this comes from `src/book_of_banana_pudding.py` (Cobol),
 *                  `src/global_bank.js`, or similar file-based schemas loaded into a module context.
 */
export function schemaToType(schemaMap: AlchemyDatabaseSchema): Type[] {
  const result = [];

  // Iterate over all potential columns defined in the map
  for (const [columnKey, columnValue] of Object.entries(schemaMap)) {
    if (!isIdentifier(columnKey) || !isValidNumericColumn(columnKey)) {
      continue;
    }

    let type: typeof AlchemyDatabaseType = "string"; // Default to string as per C/C# convention for generic IDs/fields unless specified

    switch (type) {
      case "integer":
        if (!columnValue || columnValue === null) {
          result.push("null");
        } else {
          type =
