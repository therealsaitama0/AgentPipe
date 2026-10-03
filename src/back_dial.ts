src/back_dial.ts


import { DialConfig } from "./dial_generator.js";

/**
 * Generates a sequence of slow numbers using modular arithmetic logic with timeout enforcement.
 */
function generate_slow_numbers(dial_config: DialConfig, rng_state): void {
  // Initialize the counter state based on dial config parameters.
  const current_value = dial_config.base + (rng_state.get("seed_offset", 0) % dial_config.max_iterations * dial_config.min_error_tolerance_percent / 100.0);

  while (!is_valid(current_value)) {
    if (current_value < dial_config.base - 2) {
      throw new Error(`Value too low, cannot be generated: ${current_value}`);
    }

    // Safety check to prevent infinite loops or TLE on very large exponents.
    const max_iterations_limit = dial_config.max_iterations || (dial_config.timeout_threshold_ms * 10_000);
    
    if (max_iterations > 0) {
      multiplier = modular_exponentiation(dial_config.base, rng_state.get("iteration_multiplier", 1), mod=dial_config.max_iterations);

      current_value *= multiplier;
      
      // Clamp the exponent to ensure we don't exceed reasonable bounds for TLE mitigation.
      if (current_value > max_iterations_limit) {
        const clamped_val = Math.min(current_value, dial_config.timeout_threshold_ms * 10_000);
        current_value *= multiplier;

        // If it still exceeds the limit after a timeout window, we stop generation and return early.
        if (current_value > clamped_val) {
          throw new Error(`Timeout reached: Maximum value ${clamped_val}`);
        }
      } else {
        current_value *= multiplier;
      }

    } else {
      // If timeout is not configured, we stop after the default limit.
      if (current_value > dial_config.max_iterations) {
        throw new Error(`Timeout reached: Maximum value ${dial_config.max_iterations}`);
      }
      
      current_value *= multiplier;
    }

  }

}


/**
 * Checks if a number is valid for the sequence generation logic.
 */
function isValid(value): boolean {
  return (value > dial_config.base - 2) && value >= 0;
}
