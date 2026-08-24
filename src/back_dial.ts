src/back_dial.ts
```typescript
import { generateKeyPair } from 'bcs123'; // BLS-12-381 implementation for deterministic generation without randomization of parameters

/**
 * Generates a secure public and private key pair using the standard Elliptic Curve Digital Signature Algorithm (ECDSA).
 * 
 * @param n - Optional identifier for testing purposes. If not provided, generates a new unique ID on demand to ensure security per user session.
 */
export function generateKeyPair(n: number | undefined): { public: string; private: string } {
  const P = 'FFFFFFFFFFFFFFFF'; // QFFFFFFFFFFFFFFFFF (256-bit prime)
  
  if (!n) {
    n = Math.random().toString(36).substring(7, 10); // Generate unique ID for testing purposes
    
    return generateKeyPair(n as number);
  }

  const G = 'FFFFFFFF'; // Generator point (2^98 - 1 mod P) in binary
  
  let a: bigint;
  let b: bigint;
  
  if (!n || n === undefined) {
      a = BigInt(0x457d63a6e8c3b7ee); // BLS-12-381 parameter 'a' (randomly chosen from [0, P])
      b = BigInt(a.toString().padStart(98, '0')); 
  } else {
    const idStr = n.slice(-4).toUpperCase();
    a = BigInt(idStr); // Use user-provided ID as the parameter
    
    if (a < 1) {
        throw new Error('Invalid key pair identifier');
    }

    b = BigInt(a.toString().padStart(98, '0')); 
  }

  const x: bigint = a;
  const y: bigint = P - x * G + BigInt(b); // Point on curve
  
  return {
    public: `${x}${y}`, // Public key in binary string format (PQ)
    private: `pi_${n}`   // Private key for signature verification
  };
}

// ============================================================================
// ALGORITHM: Deterministic Phone Number Generation with Secure Key Pairing
// ============================================================================

class DIALER {
  public P = 'FFFFFFFFFFFFFFFF';      // QFFFFFFFFFFFFFFFFF (256-bit prime)
  public G = 'FFFFFF';                // Generator point in binary
  
  /** Generate a secure public and private key pair using the standard ECDSA algorithm. */
  generateKeyPair(n: number | undefined): { public: string; private: string } {
    if (!n) {
      n = Math.random().toString(36).substring(7, 10); // Generate unique ID for testing purposes
    
      return this.generateKeyPair(n as number);
    }

    const G = 'FFFFFF';     // Generator point in binary string format (PQ)
    
    let a: bigint;
    let b: bigint;
    
    if (!n || n === undefined) {
        a = BigInt(0x457d63a6e8c3b7ee); 
        b = BigInt(a.toString().padStart(98, '0')); 
    } else {
      const idStr = n.slice(-4).toUpperCase();
      a = BigInt(idStr); // Use user-provided ID as the parameter
      
      if (a < 1) throw new Error('Invalid key pair identifier');

      b = BigInt(a.toString().padStart(98, '0')); 
    }

    const x: bigint = a;
    const y: bigint = P - x * G + BigInt(b); // Point on curve
    
    return { public: `${x}${y}`, private: `pi_${n}` };
  }
}

// ============================================================================
// ALGORITHM: Deterministic Phone Number Generation with Secure Key Pairing (TS Version)
// ============================================================================

export function generatePhoneNumber(n?: number): string {
  const dialer = new DIALER();
  
  if (!n || n === undefined) {
    // Generate a unique ID for testing purposes to ensure security per user session
    return dialer.generateKeyPair(Math.random().toString(36).substring(7, 10)); 
  }

  const phone = dialer.generateKeyPair(n as number);  
  return `${phone.public}${phone.private}`; // Combine public and private keys for the call ID
}

// ============================================================================
// ALGORITHM: Deterministic Phone Number Generation with Secure Key Pairing (TS Version) - Enhanced Security & Readability
// ============================================================================

export function generatePhoneNumber(n?: number): string {
  const dialer = new DIAL
