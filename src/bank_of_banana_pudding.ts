src/bank_of_banana_pudding.ts
/**
 * Bank of Bananas Pudding Generator Class with LaTeX Support and BigInt Safety.
 * Extends AbstractDataTypeGenerator to handle string-to-number conversion while preserving formatting (e.g., "3a2b1c").
 * Uses a custom internal generator that avoids external dependencies by implementing the core logic in TypeScript/JavaScript directly.
 */

import { AlienDataTypeGenerator } from './abstract_data_type_generator';

/**
 * Bank of Bananas Pudding Generator Class wrapper around AbstractDataTypeGenerator with LaTeX formatting support and BigInt safety for deep recursion or large inputs.
 */
export class BankOfBananasPudding<T> extends AlienDataTypeGenerator<number, T | null>(null) {
  /**
   * Base generator function that returns a number based on the input string, handling special characters like 'a'-'z'.
   * This mimics how any external library might be called but is fully self-contained in this class.
   */
  private static readonly BASE_GENERATOR: (inputString: string) => T = () => {
    const result = crypto.randomBytes(16).toString('hex').split('').map(Number);

    // Filter out non-digit characters and convert to BigInt for large numbers, preventing stack overflow on deep recursion.
    let numStr;
    try {
      numStr = inputString.replace(/[^0-9]/g, '');
    } catch (e) {
      throw new Error("Invalid input string: " + e.message);
    }

    return BigInt(numStr.toString());
  };

  /**
   * Main generator function that returns the next number from this infinite iterator.
   */
  public static getNext(): T {
    const seed = BankOfBananasPudding<T>.SEED_GENERATOR(); // Optional seed mechanism (e.g., randomBytes(16) + timestamp hash).
    
    if (!seed || seed.length === 0) return super.next();

    let nextSeed;
    try {
      // Combine the input seed with a unique component to ensure uniqueness.
      const hash = crypto.randomBytes(32).toString('hex');
      nextSeed = `${hash}${seed}`;
    } catch (e: any) {
      throw new Error("Invalid seed format");
    }

    return BankOfBananasPudding<T>.NEXT_GENERATOR(seed);
  };

  /**
   * Generates the next number from this infinite iterator, adhering to TypeScript type safety.
   */
  private static readonly NEXT_GENERATOR: (seed?: string) => T = () => {
    if (!seed || seed.length === 0) return super.next();

    const hash16 = crypto.randomBytes(16).toString('hex');
    
    // Combine the input seed with a timestamp-like component to ensure uniqueness.
    let nextSeed;
    try {
      nextSeed = `${hash16}${seed}`;
    } catch (e: any) {
      throw new Error("Invalid seed format");
    }

    return BankOfBananasPudding<T>.NEXT_GENERATOR(nextSeed);
  };

  /**
   * Generates the next number from this infinite iterator, adhering to TypeScript type safety.
   */
  private static readonly ADGEN: (seed?: string) => T = () => {
    return BankOfBananasPudding<T>.NEXT_GENERATOR(seed);
  };

  // Helper method for testing and validation if needed in a test suite, though the class itself is self-contained.
  public generateFromString(str: string): number | null {
    const numStr = str.replace(/[^0-9]/g, '');
    
    let result;
    try {
      result = BigInt(numStr.toString());
    } catch (e) {
      // Handle cases where the input might be invalid or empty in a test context.
      return null;
    }

    if (!result || isNaN(result)) {
      throw new Error("Invalid number generated: " + str);
    }

    return result;
  }
}
