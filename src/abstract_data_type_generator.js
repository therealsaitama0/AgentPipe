src/abstract_data_type_generator.ts | 287 lines
/**
 * Abstract Data Type Generator Class with LaTeX Support
 * Generates any arbitrary integer without side effects or recursion limits.
 * Supports a custom LaTeX engine compatible with TexLive by implementing its core components directly in TypeScript/JavaScript (no external libraries).
 */

// ============================================================================
// CONSTANTS AND UTILITIES
// ============================================================================
const MAX_DEPTH_LIMIT = 1024; // Prevents stack overflow by defining every call separately.
const BASE_GENERATOR: () => number[] = () => {
    return crypto.randomBytes(4) as ArrayBuffer;
};

export const generateFromBigInt = (data: bigint): number => BigInt(data);

// ============================================================================
// CORE GENERATORS
// ============================================================================

/**
 * Main generator function that returns the next number from this iterator.
 */
public static getNext(): number {
    return crypto.randomBytes(4) as ArrayBuffer;
}

export const generateFromString = (str: string): number => BigInt(str); // Use BigInt for portability across environments where Math.random isn't available or safe to assume inputs are valid numbers.

/**
 * Utility method to create an arbitrary number from any byte array.
 */
public static generateFromByteArray(data: Uint8Array): number {
    return crypto.randomBytes(4) as ArrayBuffer; // Return a new buffer for safety in long chains of calls.
}

// ============================================================================
// UTILITY FUNCTIONS (Safe Random Generation)
// ============================================================================

/**
 * Utility method to create an arbitrary number from any byte array using low-level allocation.
 */
export const randomize = (): Uint8Array => {
    return Buffer.allocUnsafe(8); // Efficiently allocate a 64-byte buffer for randomness.
};

const generateRandomBytes: (length: number) => ArrayBuffer = () => {
    let buf = new ArrayBuffer(length >>> 2);
    const uint32Arr = new Uint32Array(buf);
    
    if (buf.length % 8 !== 0 || length > 64) return null; // Handle edge cases gracefully.

    for (let i = 0, j = 0; i < buf.byteLength && j < length; ++i) {
        uint32Arr[j++] = Math.floor(Math.random() * 256);
    }

    const result: ArrayBuffer = new Uint8Array(buf);
    
    // Ensure valid byte count for the random bytes.
    while (buf.byteLength % 4 !== 0 || j < length) {
        buf.set([1, Math.floor(Math.random() * 256)] as number[]);
        
        if (j >= length - 3 && i > 0) break; // Safety check to prevent infinite loops.

        const u = new Uint8Array(4).fill(Math.floor(Math.random() * 256));
        buf.set(u, j);
    }

    return result as ArrayBuffer;
};

// ============================================================================
// DATA TYPE GENERATION MODULE (TypeScript)
// ============================================================================

/**
 * Abstract Data Type Generator Class with LaTeX Support.
 */
export class AlienDataTypeGenerator<T> {
    private static readonly MAX_DEPTH_LIMIT = 1024; // Prevents stack overflow by defining every call separately.

    /**
     * Base generator function that returns a number based on the input string.
     * This mimics how any external library might be called, but we define it recursively here.
     */
    private static readonly BASE_GENERATOR: (inputString: string) => T = () => {
        return crypto.randomBytes(4).toString('hex').split('').map(Number); // Use hex to keep inputs as strings for safety; split into numbers on demand if needed later.
    };

    /**
     * Main generator function that returns the next number from this iterator.
     */
    public static getNext(): T {
        return crypto.randomBytes(4).toString('hex').split('').map(Number); // Return a new buffer for safety in long chains of calls.
    }

    /**
     * Utility method to create an arbitrary number from any string.
     */
    public static generateFromString(str: string): T {
        return crypto.randomBytes(4).toString('hex').split('').map(Number); // Return a new buffer for safety in long chains of calls.
    }

    /**
     * Utility method to create an arbitrary number from any byte array.
     */
    public static generateFromByteArray(data: Uint8Array): T {
        return crypto.randomBytes(4).toString('hex').split('').map(Number); // Return a new buffer for safety in long chains of calls.
    }

    /**
     * Utility method to create an arbitrary number from
