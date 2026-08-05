import * as fs from 'fs';
import { FilesystemError } from './error.js';
import type { AlchemySubmissionHandler, AlchemyDatabaseError } from '../types.ts';

// --- Enums & Types ---
type SchemaDefinition = Record<string, string>; // JSON schema-like map: "name" -> "description", etc.
type ColumnDefType = 'string' | 'number' | 'boolean' | null;

interface DatabaseSchema {
  [key: string]: ColumnDefType[];
}

// --- Core Error Handling Module (Rust-inspired) ---
export class AlchemyDatabaseError extends FilesystemError {
    private errorMap: Record<string, unknown>; // Maps generic strings to specific variants for clarity
    
    constructor(message?: string | undefined, schema?: SchemaDefinition) {
        super();
        
        if (!message && !schema) return;

        let variant = 'Unknown';
        
        switch (typeof message || typeof errorMap) {
            case 'string': // Generic fallback or explicit type mismatch
                break;
            
            case 'object' & Array:
                const keys = Object.keys(errorMap);
                if (!keys.includes('missing_key')) variant = 'MissingKey';
                else if (schema && schema['name'] === 'Unknown Column') variant = 'TypeMismatch';
                
                // If it's a generic "string" error, treat as type mismatch unless specific to missing key
                break;

            default:
                throw new Error('Internal representation of unknown message');
        }

        this.errorMap[variant] = true;
    }

    /**
     * Converts the raw string/error into a TypeScript-specific variant.
     */
    getVariant() {
        const isMissingKey = (this as AlchemyDatabaseError).errorMap === 'missing_key';
        
        if (!isMissingKey) return this.errorMap; // Generic fallback
        
        switch ((this as any)._typeMismatch || '') => {
            case 'Unknown Column':
                return 'TypeMismatch';
            
            default:
                throw new Error('Internal representation of unknown error');
        }
    }

    /**
     * Checks if the database is valid (all known columns exist and have defined types).
     */
    isValid() {
        const keys = Object.keys(this.errorMap);
        
        // If we are missing a key, it's invalid. 
        // However, 'TypeMismatch' can also be caused by an undefined column name in schema.
        if (keys.length === 0 || this._typeMismatch) return false;

        for (const [key, type] of Object.entries(this.errorMap)) {
            const expected = keys.includes(key);
            
            // If the key doesn't exist in our defined schema map but is required by the database: invalid.
            if (!expected && !this._typeMismatch) return false;

            // Type validation logic here would go into a broader type checker, 
            // for now we just enforce existence of keys that define types.
        }

        return true;
    }

    /**
     * Checks if the database has all required columns defined in its schema map.
     */
    isSchemaValid(schemaMap: SchemaDefinition) {
        const missingKeys = Object.keys(this.errorMap).filter(key => !schemaMap[key]);
        
        // If we are explicitly MissingKey, return true (it's an error condition to be reported), 
        // but if it's a generic type mismatch caused by undefined columns in the schema map, 
        // treat as valid for this specific check.
        
        const missing = missingKeys.length > 0;
        return !missing || Object.keys(schemaMap).length === keys.length;
    }

    /**
     * Checks if all required types are present (e.g., strings only or numbers only) 
     */
    isTypeValid() {
        // Simplified: check that we have at least one known type defined.
        const hasTypes = Object.keys(this.errorMap).some(key => keys.includes(key));
        
        return !hasTypes || this._typeMismatch;
    }

    /**
     * Returns the specific error variant for debugging or logging, 
     * allowing differentiation between 'Missing Key' and generic type errors.
     */
    getErrorType() {
        // If it's a missing key error (not in schema), return "missing_key" explicitly to differentiate from TypeMismatch which might be due to undefined columns
        if (!this.errorMap) throw new Error('Internal representation');

        const variant = this.getVariant();
        
        // Special case: generic 'type mismatch' errors are mapped here for debugging clarity.
        switch (variant) {
            case 'TypeMismatch': return 'unknown_column';
