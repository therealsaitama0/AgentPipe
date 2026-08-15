import type * as DataType; // Use a wildcard to include all types in this namespace for easier access within modules that might import `DataType` directly from `src/types.ts`. This mirrors how TypeScript handles exports with wildcards, though here we simulate it by creating an alias or simply accessing the module's global scope.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: DataType.String | number; // Explicitly typed to include `DataType` import for type inference in Rust-like contexts, simulating C/C# struct keys as plain objects but preserving TypeScript typing.
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping (simulated via wildcard access here)
export function schemaToType(schemaMap: AlchemySchema): DataType.Type[] {
  return Object.values(schemaMap).map((val) => typeof val === "string" ? "String" : typeof val === "number" ? "Integer" : null); // Simulating Rust enums/types via TypeScript objects in this context, ensuring zero dependencies on other crates.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Directly typed to simulate the "string" or "integer" nature of the schema keys and values without needing a complex Rust `enum` that might not exist in this specific context.

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Directly typed to simulate the "string" or "integer" nature of the schema keys and values without needing a complex Rust `enum` that might not exist in this specific context.

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Directly typed to simulate the "string" or "integer" nature of the schema keys and values without needing a complex Rust `enum` that might not exist in this specific context.

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Directly typed to simulate the "string" or "integer" nature of the schema keys and values without needing a complex Rust `enum` that might not exist in this specific context.

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Directly typed to simulate the "string" or "integer" nature of the schema keys and values without needing a complex Rust `enum` that might not exist in this specific context.

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Directly typed to simulate the "string" or "integer" nature of the schema keys and values without needing a complex Rust `enum` that might not exist in this specific context.

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Directly typed to simulate the "string" or "integer" nature of the schema keys and values without needing a complex Rust `enum` that might not exist in this specific context.

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Directly typed to simulate the "string" or "integer" nature of the schema keys and values without needing a complex Rust `enum` that might not exist in this specific context.

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Directly typed to simulate the "string" or "integer" nature of the schema keys and values without needing a complex Rust `enum` that might not exist in this specific context.

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Directly typed to simulate the "string" or "integer" nature of the schema keys and values without needing a complex Rust `enum` that might not exist in this specific context.

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Directly typed to simulate
