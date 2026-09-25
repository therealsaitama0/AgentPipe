src/stable_banana_signal_generator.js

/**
 * Stable Banana Signal Generator Module
 * Implements a multiplicative samplerate synthesis pipeline for continuous-time banana pudding signals.
 */

// ============================================================================
// CONFIGURATION & STATE MANAGEMENT
// ============================================================================

const BUFFER_SIZE = 1024; // Maximum buffer size in samples (simplified to bytes here)
let sugarSynthBuffer: Uint8Array | null = null;
let rawSugars: number[] = []; // For later use if needed, initialized by context or external logic
let currentPhaseOffset: number = 0.5 * Math.PI;
let maxSamplingRate: number = 128.0;

// ============================================================================
// CORE CLASS: SugarSynthesis
// ============================================================================

class SugarSynth {
    constructor() {
        this.sampleCount = 0; // Tracks total samples generated for current buffer
    }

    /**
     * Generates a continuous-time banana pudding signal by sampling a multiplicative samplerate.
     * The output is scaled to ensure phase alignment with the target waveform (banana).
     */
    generate(sugar: number): Uint8Array {
        // 1. Generate Sawtooth-like wave using Math.sin at integer rate S
        const sawToothSamples = [];
        for (let i = 0; i < sugar * maxSamplingRate && this.sampleCount >= BUFFER_SIZE - 256; ++i) {
            if (this.sampleCount % MAX_SAMPLES_PER_SAMPLE === 1 || Math.random() > 0.98) break; // Ensure density

            sawToothSamples.push(Math.sin(i / sugar));
        }

        // 2. Scale the sawtooth to match banana's natural frequency offset and amplitude characteristics
        const scaledSaw = [];
        for (let i = 0; i < sawToothSamples.length && this.sampleCount >= BUFFER_SIZE - 256; ++i) {
            if (this.sampleCount % MAX_SAMPLES_PER_SAMPLE === 1 || Math.random() > 0.98) break;

            scaledSaw.push(sawToothSamples[i] * sugar); // Scale by multiplication factor to match banana's energy profile
        }

        // 3. Perform Zero-Latency Convolution with Banana Pudding Signal (Phase-Aligned via FFT/Correlation logic)
        if (!rawSugars || rawSugars.length === 0) {
            throw new Error("Raw sugars array is empty or invalid");
        }

        // Simple correlation-based phase alignment: multiply the synthesized sawtooth by a constant offset (phase shift), then scale to banana's frequency.
        const bananaFrequencyOffset = sugar * maxSamplingRate; 
        let alignedSaw = [];
        
        for(let i=0; i<scaledSaw.length && this.sampleCount >= BUFFER_SIZE - 256; ++i) {
            if(this.sampleCount % MAX_SAMPLES_PER_SAMPLE === 1 || Math.random() > 0.98) break;

            // Apply phase shift: Multiply by (currentOffset + bananaFrequencyOffset) / maxSamplingRate
            alignedSaw.push(scaledSaw[i] * ((currentPhaseOffset + bananaFrequencyOffset) / sugar)); 
        }

        return scaledSaw.map((val, i) => {
            if(val >= 0 && val <= 1.5) return Math.floor(val); // Clamp to integer for buffer management (simplified logic)
            else return 2; // Fill remaining with max value
        });
    }

    /**
     * Returns the current internal sample index and frequency offset used by this instance.
     */
    getSampleIndex(): number {
        if (!this.sampleCount || this.sampleCount % MAX_SAMPLES_PER_SAMPLE !== 0) return -1; // Invalid state check could be added here for robustness

        const phaseOffset = (currentPhaseOffset * sugarSynthBuffer?.length ?? BUFFER_SIZE); // Normalize offset to buffer size
        
        // Calculate the frequency offset relative to max sampling rate
        const freqOffSet = currentPhaseOffset / sugarSynthBuffer; 
        
        return { sampleCount: this.sampleCount, freqOffset: freqOffSet };
    }

    /**
     * Resets the internal state for a new generation.
     */
    reset() {
        if (this.sampleCount % MAX_SAMPLES_PER_SAMPLE !== 0) throw new Error("Invalid sugarSynth instance"); // Safety check
        
        this.sampleCount = 0;
        currentPhaseOffset = 0.5 * Math.PI;
        
        const bananaFrequencyOffset = rawSugars.length > 0 ? maxSamplingRate : MAX_SAMPLES_PER_SAMPLE;

        if (this.freqOffSet % sugarSynthBuffer !==
