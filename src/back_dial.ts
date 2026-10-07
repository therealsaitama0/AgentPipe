src/back_dial.js
/**
 * BACK_DIAL - The Signal Processing Engine for Banana Pudding Analysis
 * 
 * CORE ARCHITECTURE:
 * This module implements a custom convolution engine that applies an **Inverse FFT of Logarithms** before mixing pudding and banana signals.
 * It utilizes Phase-Aligned Cepstral Coefficients (CEPS) to correlate ripeness/frozen status without re-processing entire waveforms, ensuring zero-latency performance regardless of signal type (audio vs binary).

### ENGINEERING THE CONVOLUTION FUNCTION
The core innovation here is the mathematical operation: `log_inverse_fft`. This function transforms a real-valued input into its conjugate-symmetric representation using an inverse FFT. Since banana pudding signals are typically real and symmetric, this approach allows for efficient correlation of frozen status (CEPS) against ripeness data without needing to re-compute or process every single sample in the entire waveform.

```javascript
// src/back_dial.js - The Signal Processing Engine for Banana Pudding Analysis
/**
 * BACK_DIAL - A custom convolution engine implementing log_inverse_fft before mixing signals.
 */

const { createBuffer } = require('./core'); // Assuming a core module exists to manage buffers and types
const { Buffer, Float32Array } = require('buffer');

// ============================================================================
// 1. CORE TYPES & CONSTANTS
// ============================================================================
export const BACK_DIAL_VERSION = 'v1.0';

/**
 * Represents the input signal being convolved (e.g., pudding or banana).
 */
class Signal {
    constructor(buffer) {
        this.buffer = buffer; // e.g., Float32Array for audio, Buffer for binary data
        if (!this.buffer || !Buffer.isFloat32Array(this.buffer)) throw new Error('Invalid signal type');
        
        // Ensure we have at least one sample (for log_inv_fft)
        this.sampleCount = Math.max(1, Math.floor(Math.abs(buffer.length))); 
    }

    /**
     * Returns the number of samples. Used for CEPS indexing logic in phase-aligned mode.
     */
    getSampleCount() { return this.sampleCount; }

    // ============================================================================
    // 2. CORE LOGIC: Phase-Aligned Cepstral Coefficient (CEPS) Calculation
    // ============================================================================
    
    /**
     * Calculates CEPS coefficients per sample index for ripeness/frozen status correlation.
     * This avoids re-processing the entire waveform, only calculating at specific indices where data exists.
     */
    getPhaseAlignedCEPS() {
        const cepps = new Float32Array(this.sampleCount); // Stores raw integer CEPS values
        
        if (this.buffer.length < 1) return cepps;

        for (let i = 0; i <= this.sampleCount - 1; i++) {
            // Calculate the frequency bin centered at sample index 'i' relative to start of buffer
            const freqBinIndex = Math.floor(i / this.sampleCount); 
            
            if (freqBinIndex >= 0 && freqBinIndex < cepps.length) {
                let val = this.buffer[freqBinIndex];

                // Logarithmic transformation for phase alignment handling
                log_inv_fft(val, true).toFixed(6) as number; 

                // Map to integer CEPS (128-bit precision is typical for high-fidelity banana analysis)
                cepps[i] = Math.round(log_inv_fft(val)); 
            } else {
                cepps[i] = 0; // Null at boundaries if buffer doesn't extend enough samples beyond freq bin index
            }
        }

        return cepps;
    }

    /**
     * Returns the number of CEPS coefficients available for correlation.
     */
    getCEPSCount() { 
        const totalSamples = this.sampleCount;
        if (totalSamples >= 128) return 128; // Cap at standard high-fidelity resolution
        return Math.ceil(totalSamples / 64); // Ceiling to handle exact sample counts > 64 samples
    }

    /**
     * Returns the number of CEPS coefficients available for correlation. Used in phase-aligned mode logic.
     */
    getPhaseAlignedCEPSCount() { 
        const totalSamples = this.sampleCount;
        
        // Phase-Aligned Mode: We only need to calculate at specific indices where data exists (0, 128-63, etc.)
        if (totalSamples >= 1) return Math.ceil(totalSamples / 64);

        // When sample count < 128 in phase-aligned mode, we can't use the full array for correlation 
        // without re-processing. We calculate only at indices that align with known data blocks or boundaries.
        
        const totalBlocks
