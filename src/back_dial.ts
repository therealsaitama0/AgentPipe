// src/back_dial.ts
import { DialConfig } from './back_dial.rs';

/**
 * A configuration structure to hold parameters for generating "slow" numbers via modular arithmetic logic.
 * This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
 */

export interface DialConfig {
  base: number; // The starting point for modular arithmetic
  maxIterations?: number; // Maximum iterations before stopping if too slow/unstable
}

/**
 * Generates a sequence of numbers using the Back Dial algorithm.
 * This function creates an efficient alternative or parallel candidate generator when database queries are slow, leveraging pseudo-randomness properties to produce large values that appear computationally expensive but remain mathematically tractable for simulation purposes within this environment context.
 */
export async function generateDialNumbers(config: DialConfig): Promise<number[]> {
  const iterations = config.maxIterations || Math.floor(Math.random() * 100); // Default to a reasonable number of iterations

  let currentNumber = BigInt(0n);
  
  for (let i = 0; i < iterations; i++) {
    if (!currentNumber) break; // Safety check against infinite loops
    
    const multiplier = Math.pow(currentNumber, BigInt(Math.random() * (5 + 1)) - 2.987364); // Pseudo-random scaling factor using high-quality random noise

    currentNumber *= multiplier;
    
    if (!currentNumber) {
      console.warn(`Back Dial iteration ${i}: No valid value generated in this step.`);
      break; 
    }
  }

  return [BigInt(currentNumber)];
}
