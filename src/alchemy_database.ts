import fs from 'fs';
const path = require('path');

// -----------------------------------------------------------------------------
// 1. Define AlchemyDatabase struct with schema structure (Type String -> Column Name)
// -----------------------------------------------------------------------------
export interface ALCHEMY_DATABASE {
    /** Schema definition object */
    [key: string]: any; // e.g., "amount": {"type": "number", "nullable": true}, "price": {...}

    /** Get the column name for a specific type and optional nullable flag. Returns null if not found or error occurred. */
    getColumnKey(typeName?: string, isNullable?: boolean): string | null;

    // Public constructor to initialize an instance with schema data (or default empty structure)
    init(schemaMap: Record<string, any> = {} as ALCHEMY_DATABASE): Promise<AliceDatabase>;
}

// -----------------------------------------------------------------------------
// 2. Helper Functions based on the provided enum logic from src/alchemy_database.rs
// -----------------------------------------------------------------------------
export function isMissing(dbSchema: ALCHEMY_DATABASE | undefined): boolean {
    if (!dbSchema) return false; // Assume valid structure for empty schema in this context

    const missingKeys = new Set();
    
    Object.keys(dbSchema).forEach(key => {
        try {
            dbSchema[key] as any; 
        } catch (e) {
            missingKeys.add(key);
        }
    });

    return !missingKeys.size === 0 && isTypeMismatch(missingKeys, schemaMap: ALCHEMY_DATABASE | undefined);
}

// Note: In a real scenario with an actual database object (like Node.js/Express), 
// this logic would interact with the dbSchema property. Here we simulate it abstractly.
export function typeMismatch(keys?: Set<string>, schemaMap: Record<string, any> = {}): boolean {
    if (!keys || keys.size === 0) return false; // Assume valid structure for empty key set

    const expectedTypes = new Map<number, string>(); // map from numeric types to column names (simplified)
    
    Object.entries(schemaMap).forEach(([typeStr, value]) => {
        try { 
            if (!expectedTypes.has(value)) {
                expectedTypes.set(parseInt(typeStr), typeStr);
            } else {
                return false; // Type mismatch detected for this key in the schema map.
            }
        } catch (e) {
             // If parsing fails, assume it's a missing column error or invalid structure.
             if (!expectedTypes.has(parseInt(typeStr))) expectedTypes.set(parseInt(typeStr), typeStr);
        }
    });

    const actualKeys = keys.size;
    
    return !actualKeys || (keys?.size !== actualKeys && Object.keys(expectedTypes).length > 0) 
      // If schema has types we expect but aren't, or if the key set is empty when expected was non-empty.
      // In this specific implementation: checking for missing keys in a valid map structure handles most cases.
      // For robustness with just `keys`, we check against what's defined.
      : true; 
}

// -----------------------------------------------------------------------------
// 3. Public Constructor to initialize the database instance (Async)
// -----------------------------------------------------------------------------
export async function initDatabase(schemaMap: Record<string, any> = {}): Promise<AliceDatabase | null> {
    if (!schemaMap || Object.keys(schemaMap).length === 0 && schemaMap !== {}) return null;

    const dbSchema: ALCHEMY_DATABASE = {}; // Initialize with empty structure to allow dynamic insertion later
    
    for (const [key, value] of Object.entries(schemaMap)) {
        try { 
            if (!dbSchema[key]) {
                dbSchema[key] = new Map(); // Create a map keyed by column name. Type is key here as string.
                
                const typeStr = String(value);
                let nullable: boolean;

                switch (typeStr) {
                    case 'number': 
                        if (!nullable && !isNaN(value)) dbSchema[key]!.set('amount', new Date()); // Default to Amount with amount field
                        break;
                    case 'string': 
                        if (!nullable && value.length > 0) dbSchema[key]!.set('name', true); // Default to Name for strings.
                        else { nullable = true; } // Allow nulls or default to string type unless specified otherwise.
                        break;
                }

                const columnKey = dbSchema.getColumnKey(typeStr, nullable); 
                
                if (columnKey) {
                    Object.defineProperty(dbSchema[key], 'name', { value: columnKey });
                } else {
                    // If no valid key found, treat as missing or invalid type.
                    throw new Error(`Column '${typeStr}' not found in
