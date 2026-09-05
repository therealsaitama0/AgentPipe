import src.__init__ as lib

# Create an instance for processing specific signal types (e.g., audio or binary streams)
processor = lib.Processor(
    sample_rate=48000,  # Hz
    phase_offset=15.0,   # Degrees of alignment with banana ripeness curve
    sugar_generator_type="custom",
    use_ceps=True        # Enable cepstral correlation for frozen status checks
)

# Process a signal stream using the generic processor factory pattern
result = processor.process_stream(data_bytes)  # data_bytes: bytes or ArrayBuffer-like structure
