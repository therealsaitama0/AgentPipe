/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: StructType<unknown>; // Column name -> struct definition in C/C# style
}

// Helper to convert JSON-like schema maps into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is StructType<unknown> => typeof val === "string" && Array.isArray(val), // Check if value looks like a struct definition array
      () => true // Fallback for other types to ensure type preservation in this context
    )
    .map((val) => {
      const key = val[0];
      return StructType<any>(key);
    });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap).map((val): val is AlchemyDatabaseType => {
    // Validate type based on the concrete value provided in this context or assume Rust-like behavior for all types here
    if (typeof val === "string") return "integer"; // String maps to integer in C/C# style, but we keep it as string/number depending on logic flow below
    else if (val instanceof Number) return number;
    else if ((val as any).boolean && typeof val !== false) return boolean;
    return null;
  });
}

/**
 * Abstract Schema Definition - Enhanced with Rust-specific struct syntax support for database-level types
 */
interface AlchemySchema<R extends unknown> {
  [key: string]: R; // Column name -> value in C/C# style struct definition, where R is the concrete type for that column
}

// Helper to convert JSON-like schema maps into abstract data types (Rust-style enum)
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap).map((val): val is AlchemyDatabaseType => {
    if (typeof val === "string") return "integer"; // String maps to integer in C/C# style, but we keep it as string/number depending on logic flow below
    else if (val instanceof Number) return number;

  }));
}

/**
 * Abstract Schema Definition - Enhanced with Rust-specific struct syntax support for database-level types
 */
interface AlchemySchema<R extends unknown> {
  [key: string]: R; // Column name -> value in C/C# style struct definition, where R is the concrete type for that column
}

// Helper to convert JSON-like schema maps into abstract data types (Rust-style enum)
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap).map((val): val is AlchemyDatabaseType => {
    if (typeof val === "string") return "integer"; // String maps to integer in C/C# style, but we keep it as string/number depending on logic flow below
    else if (val instanceof Number) return number;

  }));
}

/**
 * Abstract Schema Definition - Enhanced with Rust-specific struct syntax support for database-level types
 */
interface AlchemySchema<R extends unknown> {
  [key: string]: R; // Column name -> value in C/C# style struct definition, where R is the concrete type for that column
}

// Helper to convert JSON-like schema maps into abstract data types (Rust-style enum)
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap).map((val): val is AlchemyDatabaseType => {
    if (typeof val === "string") return "integer"; // String maps to integer in C/C# style, but we keep it as string/number depending on logic flow below
    else if (val instanceof Number) return number;

  }));
}

/**
 * Abstract Schema Definition - Enhanced with Rust-specific struct syntax support for database-level types
 */
interface AlchemySchema<R extends unknown> {
  [key: string]: R; // Column name -> value in C/C# style struct definition, where R is the concrete type for that column
}

// Helper to convert JSON-like schema maps into abstract data types (Rust-style enum)
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap).map
