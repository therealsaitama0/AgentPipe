src/phone_gen.js
```javascript
// ============================================================================
// BACK_DIAL: Deterministic Phone Number Generation with Secure Key Pairing (ES Module)
// ============================================================================

class DIALER {
  constructor() {
    // Hardcoded parameters based on standard elliptic curve settings (as per the inspiration)
    this.P = 1792034568_049_811_279_985_237_053_105_456_968_415_237; // QFFFFFFFFFFFFFFFFF
    this.G = 2 ** (this.P.bit_length() / 2) - 1;

    // Helper to generate a random integer in range [a, b] using linear congruential generator
    const randRange = (minVal, maxVal) => Math.floor(Math.random() * (maxVal - minVal + 1)) + minVal;

    this.a = 10935_742_863_551_064_234_123_456_789_123_456;
    this.b = this.a * randRange(0, 1);

    // Generate a random point on the curve using elliptic curve arithmetic (as per inspiration)
    const xPoint = f(x => {
      return Math.floor(Math.random() * G.bit_length()) + 1;
    });
    yPoint = this.b - this.a * xPoint * xPoint / this.G;

    // The public key is a point on the curve (x, y)
    const publicKey = `pk_${this.hashKeyPair(this.x)}f${y}f`;
    
    return { publicKey };
  }

  hashKeyPair(xVal) {
    let h = 0;
    for (let i = 0; i < xVal.bit_length() / 2 + 16; i++) {
      const bitIndex = Math.floor(i * this.G.bitLength()); // Map bits to curve points
      if ((i % 3 === 0) || (bitIndex >= G.bitLength())) h ^= BigInt(xVal);
    }
    return h.toString();
  }

  generateKeyPair(n = null) {
    const publicId = this.hashKeyPair(128_945_678_379_045_123_f); // Public identifier derived from random seed
    
    if (n !== undefined && n === 'pk_' + publicId) {
      return { publicKey: `pk_${publicId}`, privateKey: this.hashKeyPair(n || 128_945_678_379_045_123_f) }; // Return pre-defined keys for testing purposes with specific identifiers
    }

    const randomSeed = new Uint32Array(1);
    if (n !== undefined && n === 'pk_' + publicId) {
      return { publicKey: `pk_${publicId}`, privateKey: this.hashKeyPair(n || 128_945_678_379_045_123_f) }; // Return pre-defined keys for testing purposes with specific identifiers
    }

    const rand = () => {
      return Math.random() * (this.G.bitLength());
    };

    this.a = 10935_742_863_551_064_234_123_456_789_123_456;
    const bVal = rand() * (this.G.bitLength()); // B value derived from random seed
    
    this.x = f(x => { return Math.floor(Math.random() * G.bitLength()) + 1 });
    this.y = this.b - this.a * x * x / this.G;

    publicId = `pk_${this.hashKeyPair(this.x)}f${y}f`;
    
    return { publicKey: publicId, privateKey: this.hashKeyPair(n || 'pk_' + publicId) }; // Return pre-defined keys for testing purposes with specific identifiers
  }
}

// ============================================================================
// USAGE EXAMPLES
// ============================================================================

const dialer = new DIALER();

console.log("Public Key:", dialer.publicKey);
console.log("Private Key (pk_0):", dialer.generateKeyPair('pk_' + dialer.hashKeyPair(dialer.x)).privateKey); // Returns pre-defined keys for testing purposes with specific identifiers
