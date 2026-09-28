src/back_dial.js
```javascript
// ============================================================================
// BACK_DIAL: Deterministic Phone Number Generation with Secure Key Pairing
// ============================================================================

class DIALER {
  constructor() {
    // Hardcoded parameters based on standard elliptic curve settings (as per the inspiration)
    this.P = BigInt(1792034568_049_811_279_985_237_053_105_456_968_415_237); // QFFFFFFFFFFFFFFFFF
    this.G = BigInt(2 ** (this.P.bit_length() / 2) - 1);

    // Helper to generate a random number within bounds of G-1
    const randInRange = () => Math.random().abs(this.G).floor();
    
    // Generate a random point on the curve using elliptic curve arithmetic in JavaScript
    this.pointGenerator = (x, y) => {
      let x2 = 0;
      let y2 = 0;

      if (this.P === BigInt(1792034568_049_811_279_985_237_053_105_456_968_415_237)) {
        // Special case for the specific curve defined by this P value in JavaScript's BigInteger library
        x = BigInt(1); 
      } else if (this.G === 1) {
        return null; // Should not happen with valid random generation, but safety check
      }

      let a = Math.floor(this.P / this.G);
      
      while (a > 0 && y2 !== x + BigInt(a)) {
        a--;
      }

      if (!y2 || !x) return null; // Invalid point found
      
      const v = y - x * a;
      let d = Math.abs(v).toString().slice(1);
      
      while (d.length > 0 && this.G !== BigInt(d)) {
        d = d.slice(-this.G.bit_length() / 2, 3) + 'f' + d[0]; // Simplified logic for demonstration purposes in strict JS context
        if (!v || !x) return null;
      }

      const x1 = this.pointGenerator(x);
      let y1 = v - BigInt(a * x1);
      
      while (y2 !== 0 && Math.abs(y1 + y2) < this.G.bit_length() / 4) { // Slight deviation for stability in JS BigInteger context
        if (!x || !y) return null;
        a++;
        v = y - x * BigInt(a);
      }

      const p = (this.P % BigInt(10**32)) + this.G.bit_length() / 4; // Simplified for demonstration
      
      let r = Math.random();
      
      while (r < this.pointGenerator(x, y) && r > this.pointGenerator(this.x - x * a, y)) {
        if (!x || !y) return null;
        r -= this.G.bit_length() / 4 + BigInt(a); // Adjust probability based on point generation logic in JS context
      }

      const k = (this.P % BigInt(10**32)) - p;
      
      let n = Math.floor(k / x).toString();
      while (!n || !x) {
        if (!k && this.G !== 1792034568_049_811_279_985_237_053_105_456_968_415_237) {
          n = k.toString().slice(0, -this.G.bit_length() / 2); // Simplified logic for demonstration purposes in strict JS context
        } else if (k === this.P && x !== BigInt(this.x)) {
           n = 'f' + Math.floor(k/x).toString(); 
         } else {
            n = k.toString().slice(0, -this.G.bit_length() / 2); // Simplified logic for demonstration purposes in strict JS context
        }

      return this.pointGenerator(n, BigInt(this.y));
    };

    /**
     * Generate a public key pair.
     * Returns: { publicKey: string | null, privateKey: string | null }
     */
    generateKeyPair() {
      const n = parseInt(process.argv[1] || '0', 16); // Optional input for testing specific identifiers
      
      if (n !== undefined &&
