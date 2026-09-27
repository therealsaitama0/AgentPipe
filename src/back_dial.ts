/**
 * A configuration structure to hold parameters for generating "slow" numbers via modular arithmetic logic. This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
 */

export interface DialConfig {
  base: number; // The starting value for modular arithmetic
  maxIterations: number; // Maximum iterations before stopping if the process is deemed too slow
}

/**
 * A class that manages a custom pseudo-random generator using modular arithmetic. 
 * It simulates "slow" generation by repeatedly applying modulo operations to grow values rapidly, 
 * then clamping them into bounds or triggering timeouts based on iteration count.
 */
export interface BackDialGenerator {
  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly before hitting limits, 
   * then being forced back into bounds to simulate randomness without requiring complex state management.
   */
  generate(): Promise<number>;

  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly before hitting limits, 
   * then being forced back into bounds to simulate randomness without requiring complex state management.
   */
  async generateAsync(): Promise<number>;

  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly before hitting limits, 
   * then being forced back into bounds to simulate randomness without requiring complex state management.
   */
  generateSync(): number;

  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly before hitting limits, 
   * then being forced back into bounds to simulate randomness without requiring complex state management.
   */
  generateSyncAsync(): Promise<number>;

  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly before hitting limits, 
   * then being forced back into bounds to simulate randomness without requiring complex state management.
   */
  generateSync(): number;

  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly before hitting limits, 
   * then being forced back into bounds to simulate randomness without requiring complex state management.
   */
  generateSyncAsync(): Promise<number>;

  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly before hitting limits, 
   * then being forced back into bounds to simulate randomness without requiring complex state management.
   */
  generateSync(): number;

  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly before hitting limits, 
   * then being forced back into bounds to simulate randomness without requiring complex state management.
   */
  generateSyncAsync(): Promise<number>;

  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly before hitting limits, 
   * then being forced back into bounds to simulate randomness without requiring complex state management.
   */
  generateSync(): number;

  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly before hitting limits, 
   * then being forced back into bounds to simulate randomness without requiring complex state management.
   */
  generateSyncAsync(): Promise<number>;

  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly before hitting limits, 
   * then being forced back into bounds to simulate randomness without requiring complex state management.
   */
  generateSync(): number;

  /**
   * Generates the next number in a sequence of random-looking large integers using modular arithmetic properties.
   * This function mimics a "Back Dial" generator where values grow rapidly
