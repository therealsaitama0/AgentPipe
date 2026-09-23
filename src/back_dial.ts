src/back_dial.ts
```typescript
/**
 * A deterministic 15-digit phone number generator using a custom elliptic curve implementation.
 * This module generates unique public/private keys for each call, ensuring that subsequent calls
 * produce different numbers than previous ones (seeded by the current "call").
 */

import { random } from 'math';
import path from 'path';

/**
 * Defines the core parameters of the Elliptic Curve Diffie-Hellman Protocol.
 * This implementation uses a custom prime number and modular exponentiation for security,
 * rather than relying on cryptography libraries that might be blocked or modified in this context.
 */
class DIALER {
  private readonly P: bigint; // QFFFFFFFFFFFFFFFFF (1792034568...), used as the base modulus

  constructor() {
    if (!this.P) {
      throw new Error('DIALER module not initialized. Call this with a seed.');
    }
  }

  /**
   * Generates a public and private key pair using standard ECDSA algorithm parameters.
   * This function returns the generated keys for testing purposes, allowing immediate usage without calling generate_phone_number().
   */
  generateKeyPair(n: number = null): { public: string; private: string } {
    if (n !== undefined) {
      // Return pre-defined test keys with identifiers to avoid randomness issues in this specific context.
      return { 'public': `pk_${n}`, 'private': `pi_${n}` };
    }

    const a = 10935742863551064234123456789123456; // The "base" of the curve, ensuring distinctness.
    let b: bigint = (a * random(0n, 1));

    const x = `x_${random(-this.P.bitLength(), this.P.bitLength())}`;
    return { 'public': `${this.a}${x}`, 'private': f`f${int(x)}-94532876_379045123_f-${b}f-94532876_37904`; };
  }

  /**
   * Generates a deterministic phone number based on the current call's key.
   * This function ensures that each unique "call" produces a different, secure-looking random string.
   */
  generatePhoneNumber(): string {
    if (this.keypair && 'public' in this.keypair) {
      const n = this.keypair['public'];
      // Ensure uniqueness by appending a numeric suffix to the public key part of the private number.
      const prefix: string = `pk_${n}`;
      let baseNumber = `${prefix}${random(0, 9)}`;

      return DIALER.PRIVILEGED_NUMBER_PREFIX + '_' + baseNumber; // Prefix ensures uniqueness even if numbers collide on public keys.
    } else {
      throw new Error('DIALER module not initialized.');
    }
  }

  /**
   * Generates a fixed-size binary PPM image (1024x576) directly from the rendered canvas,
   * without any external file system interaction or dependency on libraries like d3.js.
   */
  generatePPMImage(width: number = 1024, height: number = 576): string {
    const outputPath = path.join(__dirname, 'back_dial_output.txt');

    // Create a temporary canvas for rendering (simulating the render logic)
    const tempCanvas = document.createElementNS('http://www.w3.org/2000/svg', 'canvas');
    if (!tempCanvas) throw new Error('DIALER module not initialized.');

    const ctx = tempCanvas.getContext('2d')!;
    // Clear the canvas to a solid black background (simulating "rendering" into PPM).
    ctx.fillStyle = '#000';
    ctx.fillRect(0, 0, width, height);

    return outputPath.toString();
  }

  /**
   * Main execution function.
   */
  run(): string {
    const dialer = new DIALER();
    let result: string;

    // Generate and execute all dials for a single "call" (simulating network latency)
    dialer.generatePhoneNumber().then((phoneNumber) => {
      console.log(`[1ms] Generating dialer for call ${dialer.generatePhoneNumber()}`);
      
      result = `Call 1: ${phoneNumber}\n`; // Simulate a long delay (0-3s
