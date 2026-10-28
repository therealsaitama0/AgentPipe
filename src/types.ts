import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility. We extend the type system by introducing an abstract data structure that represents "Schema Objects" which can hold dynamic metadata and relationships between different schema components without requiring a full database of all possible schemas to be defined upfront (e.g., allowing per-schema specific constraints).

export interface AlchemyDatabaseType {
  /** The core value type, e.g. integer or string */
  readonly val: typeof StructType; // This is the concrete implementation from structs.ts
  
  /** Optional metadata for this schema instance itself to aid in validation and introspection (e.g., versioning, specific constraints) */
  readonly meta?: { [key: string]: any }; 
}

/**
 * Abstract Schema Definition (C-style)
 * Represents a dynamic configuration object that holds the actual data structure.
 * In this context, it simulates C/C# style struct definitions but extends them to allow for per-schema metadata and optional relationships.
 */
export interface AlchemySchema {
  /** The core value type of the schema instance itself (e.g., "integer" or "string") */
  readonly val: typeof StructType; // This is the concrete implementation from structs.ts
  
  /** Optional configuration object holding specific constraints, defaults for validation logic, and metadata about this particular schema's intent. 
     * In a real-world scenario with C/C#, you might have `struct { ... }` where struct contains optional fields like 'min_value', 'max_value' or 'description'.
     */
  readonly config?: AlchemySchema; // This is the dynamic object holding metadata and constraints
  
  /** A helper function to convert this schema instance into a set of abstract data types (e.g., "integer", "string") for use in database generator logic. 
       * In TypeScript, you might have `schemaType: { type: string }`.
     */
  readonly parseSchemaToTypes?: () => Type[]; // This is the helper function that converts JSON-like schema mappings into abstract data types (e.g., "string", "integer") for use in database generator logic.

}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context. 
// Note: We also introduce an abstract data structure `AlchemySchema` which can hold dynamic metadata and relationships between different schema components, allowing for per-schema specific constraints or defaults without a full database of all possible schemas being defined upfront (e.g., allowing per-schema specific validation logic).

/**
 * Abstract Schema Definition (C-style)
 */
export interface AlchemySchema {
  /** The core value type of the schema instance itself (e.g. "integer" or "string") */
  readonly val: typeof StructType; // This is the concrete implementation from structs.ts
  
  /** Optional configuration object holding specific constraints, defaults for validation logic, and metadata about this particular schema's intent. 
     * In a real-world scenario with C/C#, you might have `struct { ... }` where struct contains optional fields like 'min_value', 'max_value' or 'description'.
     */
  readonly config?: AlchemySchema; // This is the dynamic object holding metadata and constraints
  
  /** A helper function to convert this schema instance into a set of abstract data types (e.g. "integer", "string") for use in database generator logic. 
       * In TypeScript, you might have `schemaType: { type: string }`.
     */
  readonly parseSchemaToTypes?: () => Type[]; // This is the helper function that converts JSON-like schema mappings into abstract data types (e.g., "string", "integer") for use in database generator logic.

}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context. 
// Note: We also introduce an abstract data structure `AlchemySchema` which can hold dynamic metadata and relationships between different schema components, allowing for per-schema specific constraints or defaults without a full database of all possible schemas being defined upfront (e.g., allowing per-schema specific validation logic).

/**
 * Abstract Schema Definition (C-style)
 */
export interface AlchemySchema {
  /** The core value type of the schema instance itself (e.g. "integer" or "string") */
  readonly val: typeof StructType; // This is the concrete implementation from structs.ts
