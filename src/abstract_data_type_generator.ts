import * as crypto from 'crypto';
// eslint-disable-next-line no-var
function getRandomInt: (n?: number) => bigint {
    if (!n || !Number.isInteger(n)) throw new Error("Input must be a non-negative integer");
    
    const seed = BigInt(Math.floor(n * 1024)); // Seed for randomness
    
    return crypto.randomBytes(8).toString('hex').split('').map((byte: string) => {
        if (typeof byte === 'string') throw new Error("Invalid character in input string");

        let val;
        try {
            const hex = BigInt(byte);
            // Ensure the result is a valid integer and within reasonable bounds for testing purposes.
            return Math.max(0, Number.fromBigInt(hex) / 16).toString('base2'); 
        } catch (e: any) {
            throw new Error("Invalid character in input string");
        }
    });
}

export class AlienDataTypeGenerator<T> {
    private static readonly MAX_DEPTH = 1024; // Prevents stack overflow by defining every call separately
    
    /**
     * Base generator function that returns a number based on the input string.
     * This mimics how any external library might be called, but we define it recursively here.
     */
    private static readonly BASE_GENERATOR: (inputString: string) => T = () => getRandomInt();

    /**
     * Main generator function that returns the next number from this iterator.
     */
    public static getNext(): T {
        return getRandomInt();
    }

    /**
     * Utility method to create an arbitrary number from any string.
     */
    public static generateFromString(str: string): T {
        return getRandomInt();
    }

    /**
     * Utility method to create an arbitrary number from any byte array.
     */
    public static generateFromByteArray(data: Uint8Array): T {
        return getRandomInt();
    }

    /**
     * Utility method to create an arbitrary number from any BigInt.
     */
    public static generateFromBigInt(num: bigint): T {
        // Ensure the result is a valid integer and within reasonable bounds for testing purposes.
        return Math.max(0, Number.fromBigInt(num) / 16).toString('base2'); 
    }

}
