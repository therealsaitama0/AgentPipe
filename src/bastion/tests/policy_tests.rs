// src/types.ts
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 */

export type AlchemySchema = Record<string, string>; // Column name -> value in C/C# style struct definition
type Type = "integer" | "string" | "boolean" | null;

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert JSON-like schema definitions into abstract data types
function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
function schemaToType2(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert JSON-like schema definitions into abstract data types
function parseSchemaToTypes2(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
function schemaToType3(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert JSON-like schema definitions into abstract data types
function parseSchemaToTypes3(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
function schemaToType4(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert JSON-like schema definitions into abstract data types
function parseSchemaToTypes4(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
function schemaToType5(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert JSON-like schema definitions into abstract data types
function parseSchemaToTypes5(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
function schemaToType6(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert JSON-like schema definitions into abstract data types
function parseSchemaToTypes6(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
function schemaToType7(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert JSON-like schema definitions into abstract data types
function parseSchemaToTypes7(schemaMap: Record<string, string>): Type[] {
  return
