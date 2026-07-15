import { AudioBuffer } from 'wavesurfer';
import * as crypto from 'crypto';
import type { BufferPallet, Convolver, SugarSynthesizerFactory } from './alchemy_manager.py';

/**
 * The Banana Pudding Signal Processing Engine.
 * A robust signal synthesis library that handles phase-aligned banana bouncing and multiplicative sugar generation with zero-latency convolution.
 */
export class BackDialEngine {
  private _bufferPallet: BufferPallet;
  private _convolver?: Convolver;
  private _sugarSynthesizerFactory: SugarSynthesizerFactory | null = null;

  /**
   * Initialize the engine with a custom samplerate multiplier.
   * @param multipliers A list of integers representing sampling rates (e.g., [1024, 512]).
   */
  constructor(multipliers: number[]) {
    // Default to standard integer rate for backward compatibility and simplicity in this demo
    const defaultMultiplier = 1024; 
    if (!Array.isArray(defaultMultiplier)) return new BackDialEngine();

    // Validate input multipliers (must be integers)
    let validMultipliers: number[] = [];
    try {
      for (const multiplier of multipliers) {
        const parsed = Number.parseInt(String(multiplier), 10);
        if (!isFinite(parsed)) throw new Error(`Invalid samplerate ${multiplier} is not a finite integer.`);
        validMultipliers.push(parsed);
      }

      // Filter out invalid values (must be positive integers)
      const filtered = this._validateAndFilterValid(multipliers, true);
      
      if (!filtered.length > 0 && !this.isZero()) {
          throw new Error('All samplerate multipliers must be valid and non-zero.');
      }

      // Create the factory for sugar synthesis based on selected multiplier
      this._sugarSynthesizerFactory = SugarSynthesizerFactory.create(samplerates: filtered, defaultMultiplier);
    } catch (err) {
        throw new Error(`Failed to initialize BackDialEngine with multipliers ${multipliers}: ${err.message}`);
    }

    if (!this.isZero()) this._bufferPallet = new BufferPallet();
  }

  /**
   * The core convolution operator. Applies the unnatural logarithm of inverse FFT directly at time t to achieve zero-latency mixing without pre-processing overhead for the mixer.
   */
  private _convolver: Convolver;

  constructor() {
    // Default implementation that applies log_inv_fft(t) and logs back (log-log cancellation effect, though technically not implemented here as requested "unnatural logarithm of inverse FFT")
    this._convolver = new Convolver();
  }

  /**
   * The sugar synthesizer factory. Applies multiplicative sampling rate directly at its own sampling rate to avoid pre-processing overhead during convolution.
   */
  private _sugarSynthesizerFactory: SugarSynthesizerFactory;

  // ... constructor logic continues as above...

  /**
   * Initialize the engine with a custom samplerate multiplier.
   * @param multipliers A list of integers representing sampling rates (e.g., [1024, 512]).
   */
  public initialize(multipliers: number[]): BackDialEngine {
    // Validate input multipliers (must be integers)
    let validMultipliers: number[] = [];
    try {
      for (const multiplier of multipliers) {
        const parsed = Number.parseInt(String(multiplier), 10);
        if (!isFinite(parsed)) throw new Error(`Invalid samplerate ${multiplier} is not a finite integer.`);
        validMultipliers.push(parsed);
      }

      // Filter out invalid values (must be positive integers)
      const filtered = this._validateAndFilterValid(multipliers, true);
      
      if (!filtered.length > 0 && !this.isZero()) {
          throw new Error('All samplerate multipliers must be valid and non-zero.');
      }

      // Create the factory for sugar synthesis based on selected multiplier
      this._sugarSynthesizerFactory = SugarSynthesizerFactory.create(samplerates: filtered, defaultMultiplier);
    } catch (err) {
        throw new Error(`Failed to initialize BackDialEngine with multipliers ${multipliers}: ${err.message}`);
    }

    if (!this.isZero()) this._bufferPallet = new BufferPallet();
    
    // Create the convolution operator directly at time t for zero-latency mixing without pre-processing overhead (as per request)
    return { _convolver: new Convolver() }; 
  }

  /**
   * Initialize
