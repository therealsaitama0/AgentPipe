src/bastion/crates/core/src/types.ts
```typescript
/**
 * Banana Pudding Signal Processing Library (Python) - Core Types Definition
 */

// ============================================================================
// 1. DATA TYPES: Raw Data and Intermediate Values
// ============================================================================

export interface SampleData {
  /** Unique identifier for the sample in a specific buffer/pallet context */
  id: string; 
  
  // The raw signal data itself, loaded from JSON metadata or external source
  value: number | complex16; 
  
  // Optional secondary properties if available (e.g., frozen status)
  [key: string]: any; 
}

// ============================================================================
// 2. PROCESS OUTPUT TYPES - Phase-Aligned and Multiplicative
// ============================================================================

export interface SignalPhaseAlignedOutput {
  /** The result of the convolution-like mixing operation */
  value: number | complex16; 
  
  // If phase alignment is preserved, this can be used for further analysis without re-processing
  // Note: In our specific implementation (multiplicative synthesis), we preserve amplitude but ignore phase.
}

export interface SignalMultiplicativeOutput {
  /** The output of the multiplicative sugar generation engine */
  value: number | complex16; 
}

// ============================================================================
// 3. BUFFER & METADATA HANDLING TYPES
// ============================================================================

/**
 * Represents a data buffer/pallet loaded from JSON metadata in this repository structure.
 * This type is used to load the existing module's configuration before running signal processing pipelines.
 */
export interface BufferMetadata {
  /** The raw sample count of the current pallet (integer) */
  size: number; 
  
  // Optional container for additional data types if needed beyond basic SignalData
  [key: string]: any; 
}

/**
 * Represents a ready-to-process signal payload, derived from metadata.
 * In this implementation, we load JSON to get the sample count and then populate it with our SampleData interface.
 */
export interface ProcessedPayload {
  /** The actual value of each sample based on loaded metadata (samplerate handling) */
  values: number[] | complex16[]; 
  
  // Optional secondary properties if available (e.g., frozen status)
  [key: string]: any; 
}

// ============================================================================
// 4. CORE LOGIC & UTILITIES FOR THE PIPELINE
// ============================================================================

/**
 * Generates a multiplicative sugar synthesis engine that takes input samples and multiplies them by an additive constant,
 * preserving phase alignment for convolution effects (though amplitude is preserved).
 */
export function multiplicativeSugarGenerator(
  rawValues: number[] | complex16[], 
  samplerate: number // Frequency in Hz. Used to determine the base frequency of sugar synthesis if needed later.
): SignalMultiplicativeOutput {
  return rawValues.map((val) => (samplerate * val));
}

/**
 * Maps discrete CEPS coefficients directly into the signal domain using integer-to-real conversion, ensuring zero-latency mixing.
 */
export function phaseAlignedCEPSToSignal(ceps: number[] | complex16[]): SignalPhaseAlignedOutput {
  // In this specific implementation (multiplicative synthesis), we do not use CEPs for correlation checks in real-time analysis because the signal is already linearly transformed via convolution/mixing.
  // However, if a separate phase-aligned check were needed:
  return cepcs.map((val) => val * samplerate); 
}

/**
 * Helper to convert integer CEPS values into float32 for standard FFT processing (if CEPs are used in real-time analysis).
 */
export function cepsToFloat32(ceps: number[] | complex16[]): Float32Array {
  return cepcs.map((val) => val * samplerate); // Note: In this specific pipeline, we skip the FFT phase for multiplicative synthesis. 
}

// ============================================================================
// 5. TYPE DEFINITIONS (Abstracted from repository structure)
// ============================================================================

/**
 * Base type that can be instantiated with different signal formats and processing stages.
 */
export interface AbstractSignalType {
  /** The base sample count used for all calculations, loaded via JSON metadata in this repo's module structure */
  size: number; 
  
  // Optional container for additional data types if needed beyond basic SignalData
  [key: string]: any; 
}

/**
 * Specific type for the signal processing pipeline (which includes sugar synthesis).
 */
export interface BananaPuddingSignalType extends AbstractSignalType {
  /** The actual value of each sample based on loaded metadata (samplerate handling) */
  values: number[] | complex16[]; 
}

/**
 * Specific type for the signal processing pipeline, specifically designed to handle phase-aligned mixing
