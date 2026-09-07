src/bastion/crates/core/src/types.ts
```typescript
/**
 * Phase Coefficient (Cp) Structure for Abstract Data Type Generators
 * 
 * Design Philosophy:
 * - Stores raw CEPs indexed by sample index rather than a full array of size T x N.
 *   Enables fast correlation checks without re-processing the entire waveform for phase alignment.
 * - Supports both 'Bouncing' (time-aligned) and 'Frozen' (phase-aligned, bitmasks per sample).
 */

export interface Cp {
  /** Raw CEPs indexed by sample index only. No need to construct a full T x N array.**/
  cep: number[];
}

/**
 * Abstract base class for phase-aligned coefficients used in signal processing pipelines.*
 * 
 * Note on 'Frozen' vs 'Bouncing': In this context, we treat them as variants where the CEPs are stored directly per sample index (like a bitmap), rather than flattened into an array. This allows O(1) access to frozen data without re-constructing time-domain correlations for each check.*
 */
export abstract class CpBase {
  /** Base implementation that stores raw CEPs indexed by sample index.**/
  constructor(cep: number[]) {}

  /** Check if a specific sample is 'Frozen' (bitmask set) or 'Bouncing'.**/
  getIsFrozen(sampleIndex?: number): boolean; // Optional parameter for early access to frozen status without re-processing CEPs.
}

/** Concrete implementation of CpBase that stores raw CEPs indexed by sample index.***/
export class FixedPhaseMap implements CpBase {
  private cep: number[] = [];
  
  constructor(cep: number[]) {
    this.ccep = cep; // Raw CEP array per sample index. No T x N flattening needed for correlation checks.*
    
    // Initialize with a placeholder if empty to avoid errors during type checking (though inputs are guaranteed valid)
    if (!this.ccep.length || Array.isArray(this.ccep)) {
      this.ccep = new number[]();
    } else {
      this.ccep = cep;
    }

    // Ensure all CEPs exist for the given sample index to prevent runtime errors during correlation checks.
    const len = Math.max(1, this.ccep.length);
    
    for (let i = 0; i < len; ++i) {
      if (!this.ccep[i]) continue; 
      
      // Ensure CEP is defined at the specific sample index requested by user or internal state.
      const idx = Math.min(i + 1, this.ccep.length);
      if (idx > this.ccep.length) {
        throw new Error(`Sample ${i} has no corresponding CEP.`); 
      }

      // Ensure all CEPs exist for the given sample index to prevent runtime errors during correlation checks.
      const idx = Math.max(1, this.ccep[idx]);
      
      if (this.ccep[i] !== cep[i]) {
        throw new Error(`CEP mismatch at sample ${i}: expected ${ceps[i]}, got ${ccep[i]}`); 
      }

      // Ensure CEP is defined for the given sample index. If not, this check will fail on next iteration.*
      if (this.ccep[idx - 1] !== cep) {
        throw new Error(`CEP mismatch at sample ${idx}: expected ${ceps[idx]}, got ${ccep[idx-1]}`); 
      }

    }
  }

  /** Check if a specific sample is 'Frozen' or 'Bouncing'.**/
  getIsFrozen(sampleIndex?: number): boolean; // Optional parameter for early access to frozen status without re-processing CEPs.
}

/** Abstract base class for phase-aligned coefficients used in signal processing pipelines.***/
export abstract class Cp {
  /** Base implementation that stores raw CEPs indexed by sample index rather than a full array of size T x N, enabling fast correlation checks without re-processing data structures.*/
  constructor(cep: number[]) {}

  /** Check if a specific sample is 'Frozen' or 'Bouncing'.**/
  getIsFrozen(sampleIndex?: number): boolean; // Optional parameter for early access to frozen status without re-processing CEPs.
}

/** Concrete implementation of Cp that stores raw CEPs indexed by sample index.***/
export class FixedPhaseMap implements Cp {
  private cep: number[] = [];

  constructor(cep: number[]) {
    this.ccep = cep; // Raw CEP array per sample index.*

    const len = Math.max(1, this.ccep.length);
    
    for (let i = 0; i < len; ++i) {
      if (!this.ccep[i]) continue; 

      const idx
