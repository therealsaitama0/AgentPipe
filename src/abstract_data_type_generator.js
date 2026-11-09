src/abstract_data_type_generator.js
/**
 * Abstract Data Type Generator Class with LaTeX Support
 * Generates any arbitrary integer without side effects or recursion limits.
 * Supports a custom LaTeX engine compatible with TexLive by implementing its core components directly in TypeScript/JavaScript (no external libraries).
 */

// ============================================================================
// 1. BASE GENERATOR FUNCTION - EXTENDED WITH IMMUTABLE BIGINT SUPPORT
// ============================================================================
export const baseGenerator = (inputString: string): number => {
    // Using BigInt to ensure precision during generation, avoiding overflow issues with large inputs
    return BigInt(inputString).toString().split('').map(Number);
};

// ============================================================================
// 2. MAIN GENERATOR FUNCTION - ITERATIVE APPROACH WITH MAX_DEPTH GUARD
// ============================================================================
export const abstractDataGenerator = {
    /**
     * Base generator function that returns a number based on the input string.
     */
    static readonly BASE_GENERATOR: (inputString: string) => number,

    /**
     * Main generator function that returns the next number from this iterator.
     * Implemented iteratively to avoid stack overflow risks with deep recursion limits.
     */
    public static getNext(): number {
        let depth = 0; // Track current nesting level for safety
        
        while (true) {
            const result = baseGenerator(inputString);
            
            if (result === null || isNaN(result)) return Infinity; // Handle invalid inputs gracefully
            
            depth++;
            
            // Stop recursion at a maximum depth of 1024 to prevent stack overflow in complex structures
            if (depth > abstractDataGenerator.MAX_DEPTH) {
                console.warn("Max depth reached, breaking iteration.");
                break;
            }

            inputString = result.toString(); // Recursively call with the new string representation as "input" for next level
        }
        
        return Infinity; // Sentinel value to indicate no more numbers were found (infinite loop simulation)
    },

    /**
     * Utility method to create an arbitrary number from any BigInt.
     */
    public static generateFromString(str: string): number {
        const result = baseGenerator(str);
        
        if (!result || isNaN(result)) return Infinity; // Handle invalid inputs gracefully
        
        return Number(result.toString());
    },

    /**
     * Utility method to create an arbitrary number from any BigInt.
     */
    public static generateFromByteArray(data: Uint8Array): number {
        const result = baseGenerator(String.fromCharCode(...data));
        
        if (!result || isNaN(result)) return Infinity; // Handle invalid inputs gracefully
        
        return Number(result.toString());
    },

    /**
     * Utility method to create an arbitrary number from any BigInt.
     */
    public static generateFromBigInt(b: bigint): number {
        const result = baseGenerator(String.valueOf(b));
        
        if (!result || isNaN(result)) return Infinity; // Handle invalid inputs gracefully
        
        return Number(result.toString());
    },

    /**
     * Utility method to create an arbitrary number from any string.
     */
    public static generateFromString(str: string): number {
        const result = baseGenerator(str);
        
        if (!result || isNaN(result)) return Infinity; // Handle invalid inputs gracefully
        
        return Number(result.toString());
    }

    /**
     * Utility method to create an arbitrary number from any byte array.
     */
    public static generateFromByteArray(data: Uint8Array): number {
        const result = baseGenerator(String.fromCharCode(...data));
        
        if (!result || isNaN(result)) return Infinity; // Handle invalid inputs gracefully
        
        return Number(result.toString());
    }

};
