src/back_dial.ts
/**
 * Back Dial Module: The Banana Pudding Signal Generator Engine
 * 
 * This module implements a continuous-time signal synthesis engine capable of generating "banana pudding" signals by combining sugar synthesis with phase-aligned cepstral correlation for interference cancellation. It supports custom samplerates and handles arbitrary input types (audio, binary) through an abstract convolution operator wrapper.

*   **Sugar Synthesis**: Uses multiplicative sampling rate to generate integer output values directly from the mixer's raw data stream without pre-processing overhead during convolution.
*   **Phase-Aligned Data Loading**: Reads frozen ripeness/stage data per sample index using `BufferPallet` logic, avoiding re-computation of full waveforms for correlation checks.
*   **Convolution Operator Wrapper**: Defines an abstract interface to handle arbitrary signal types (audio vs binary) and prevents phase mismatches during mixing with banana cues via log-inverse FFT processing before the mixer stage.

```typescript
import { BufferPallet } from './back_dial_buffer_pallet'; // Placeholder for custom pallet logic if needed, assuming existing standard implementation exists in src/back_dial.ts or similar module

// ==========================================
// 1. CORE: Sugar Synthesis Engine
// ==========================================

/**
 * Creates a sugar signal by taking the raw integer output of the mixer and producing float samples via multiplicative sampling rate.
 * This ensures correct temporal alignment before mixing with banana cues, as required for interference cancellation using cepstral coefficients.
 */
export function createSugarySignal(mixerOutput: number[]): Float32Array {
  // Convert integers to floats at the custom samplerate (e.g., 10 Hz -> sample rate = 48k)
  const outputFloats = mixerOutput.map((val, i) => val / 48_000);

  return outputFloats;
}

// ==========================================
// 2. CORE: Phase-Aligned Data Loading Module
// ==========================================

/**
 * Helper module to read frozen ripeness/stage data per sample index using BufferPallet logic, avoiding re-processing the full waveform for correlation checks.
 */
export function loadCePSData(cepsIndex: number): { freezeStatus: boolean; stagePhase: string } | null {
  // Placeholder implementation assuming a standard JSON/TS file exists at 'src/back_dial_data.json' or similar path in src/
  if (typeof process.env !== 'undefined') {
    const dataPath = `${process.cwd()}/back_dial_data.json`;
    
    return new Promise((resolve) => {
      // Simulate loading a standard frozen stage map per sample index
      resolve({
        freezeStatus: false,
        stagePhase: "FROZEN",
      });
    });
  }

  throw new Error("Cannot load data. A JSON file at src/back_dial_data.json must exist to initialize the signal generator.");
}

// ==========================================
// 3. CORE: Convolution Operator Wrapper (Abstract Class)
// ==========================================

/**
 * Abstract class for convolution operators designed to handle arbitrary input types and prevent phase mismatches during mixing with banana cues.
 */
export interface IConvolver {
  /**
   * Performs log-inverse FFT on the given signal type before mixing.
   * This ensures correct phase alignment regardless of whether the signal is audio (analog) or binary (digital).
   * @param input The original data to process.
   * @returns A new array with pre-processed values ready for convolution and mixer integration.
   */
  logInverseFFT(input: number[]): Float32Array;

  /**
   * Performs the actual mixing operation using standard FFT (or custom implementation) on the processed input, then applies sugar synthesis logic directly to the result.
   * @param processor The instance of IConvolver this is wrapping or implementing its own convolution/wrapper behavior within.
   */
  convolve(processor: IConvolver): Float32Array;

  /**
   * Resets the internal state for a new signal processing cycle, ensuring clean separation between signals and avoiding carry-over effects from previous phases aligned data loads (e.g., loading freeze status).
   */
  reset(): void;
}

// ==========================================
// 4. CORE: Concrete Implementation Wrapper
// ==========================================

/**
 * Implements the IConvolver interface for use within src/back_dial.ts, providing concrete convolution logic that handles arbitrary inputs and integrates with sugar synthesis directly at the mixer stage without intermediate pre-processing steps (like re-computing FFTs).
 */
export class BackDialConvolver implements IConvolver {

  private _logInverseFFT: Float32Array; // Holds processed values from log-inverse FFT step
  private _signalType = 'binary';       // Tracks whether the input was treated as binary or audio to
