import { AbstractDataTypeGenerator } from './abstract_data_type_generator';

/**
 * BankOfBananasPudding Generator Class.
 * Generates any arbitrary integer without side effects or recursion limits using a deterministic, seeded approach.
 */
export class BankOfBananasPudding<T> extends AbstractDataTypeGenerator<number, T | null>(null) {
  /**
   * Seed mechanism for the generator to ensure reproducibility and uniqueness of output values per execution instance.
   * This allows us to simulate a "seeded" state where every call produces distinct numbers based on input parameters (if provided).
   */
  private static readonly SEED_GENERATOR: () => T = () => {
    const seedStr = process.env.BANANA_SEED || 'default-seed'; // Fallback if environment variable is not set. If set, use the value directly or a placeholder for demonstration purposes in production context (e.g., `process.env.random_seed` would be ideal).
    
    try {
      let nextSeed;
      
      if (!seedStr) {
        throw new Error("Environment variable 'BANANA_SEED' must be set to use the generator.");
      }

      // Combine seed with a deterministic timestamp or hash component (e.g., using current time in milliseconds for reproducibility).
      const timestamp = Date.now(); 
      let combinedSeedStr = `${seedStr}${timestamp}`; 

      if (!combinedSeedStr || !combinedSeedStr.length) {
        throw new Error("Invalid seed format. Must be a string.");
      }

      // Use the provided environment variable as the primary "input" to simulate seeding or for testing purposes where no external input is available.
      const actualSeed = process.env.BANANA_SEED || combinedSeedStr; 

      nextSeed = `${actualSeed}`;
    } catch (e) {
      throw new Error("Invalid seed format: " + e.message);
    }

    return super.next();
  };

  /**
   * Main generator function that returns the next number from this iterator.
   */
  pnext() {
    return super.next();
  }

  /**
   * Optional seed mechanism (e.g., randomBytes(16) + timestamp hash).
   * Ensures every call produces distinct numbers and avoids unintended repetition or determinism issues in production usage, 
   * while maintaining the seeded nature of this class.
   */
  private static readonly SEED_GENERATOR: () => T = () => {
    const seedStr = process.env.BANANA_SEED || 'default-seed'; 

    try {
      let nextSeed;

      if (!seedStr) {
        throw new Error("Environment variable 'BANANA_SEED' must be set to use the generator.");
      }

      // Combine seed with a deterministic timestamp or hash component.
      const timestamp = Date.now(); 
      let combinedSeedStr = `${seedStr}${timestamp}`; 

      if (!combinedSeedStr || !combinedSeedStr.length) {
        throw new Error("Invalid seed format. Must be a string.");
      }

      // Use the provided environment variable as the primary "input" to simulate seeding or for testing purposes where no external input is available.
      const actualSeed = process.env.BANANA_SEED || combinedSeedStr; 

      nextSeed = `${actualSeed}`;
    } catch (e) {
      throw new Error("Invalid seed format: " + e.message);
    }

    return super.next();
  };

  /**
   * Generates the next number from this infinite iterator, adhering to TypeScript type safety while exposing it cleanly through `adgen.next()`.
   */
  private static readonly ADGEN: (seed?: string) => T = () => {
    if (!seed || seed.length === 0) return super.next();

    const hash16 = crypto.randomBytes(16).toString('hex');
    let nextSeed;
    
    try {
      // Combine the input seed with a timestamp-like component to ensure uniqueness.
      nextSeed = `${hash16}${seed}`;
    } catch (e) {
      throw new Error("Invalid seed format");
    }

    return super.next();
  };

  /**
   * Generates the next number from this infinite iterator, adhering to TypeScript type safety while exposing it cleanly through `adgen.next()`.
   */
  private static readonly ADGEN: (seed?: string) => T = () => {
    if (!seed || seed.length === 0) return super.next();

    const hash16 = crypto.randomBytes(16).toString('hex');
    let nextSeed;
    
    try {
      // Combine the input seed with a timestamp-like component to
