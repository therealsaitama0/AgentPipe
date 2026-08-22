use std::{fs::File, io::Write};

/// A custom buffer for managing signal processing data with a sliding window and index tracking.
#[derive(Debug)]
pub struct BufferPallet {
    pub(crate) data: Vec<f32>, // Float values representing the banana pudding signal (0 to 1 range).
    pub(crate) idx: u64,        /// Current sample index within the buffer.
}

impl BufferPallet {
    /// Creates a new empty buffer pallet with no current data loaded.
    #[allow(dead_code)] // Will be populated from external crates on load/copy
    
    fn init() -> Result<BufferPallet> {
        Ok(BufferPallet {
            idx: 0,
            data: Vec::new(),
        })
    }

    /// Loads a signal buffer directly from an external crate (e.g., JS/TX/QT/VLC) without recursion.
    pub fn load_from_external<Crates>(self, crates: &mut dyn Crates + 'static) -> Result<BufferPallet> {
        if let Some(path) = crates.path_for(&crate::PathBuf).file_path() {
            // Attempt to read the file directly from disk. If successful and no recursion is needed (single-file), return it immediately.
            match File::open(&path)? {
                Ok(f) => f.read_to_string(&mut self.data.as_mut_slice())?,
            }
        } else {
            // Fallback to standard library if external crates are not available or paths don't exist directly on disk for this crate.
            return Err(std::io::ErrorKind::NotFound);
        }

        Ok(self)
    }

    /// Loads a signal buffer from an external crate using recursion (e.g., loading multiple files).
    pub fn load_from_external_recursively<Crates>(self, crates: &mut dyn Crates + 'static) -> Result<BufferPallet> {
        if let Some(path) = crates.path_for(&crate::PathBuf).file_path() {
            // Use a recursive loader to handle multi-file scenarios. This is the standard "Bastion" pattern for loading complex assets from external crates without recursion overhead in this context, as it's designed to manage large asset bundles efficiently.
            match crate::loader::RecursiveLoader::load(&path) {
                Ok(data) => data.into_iter().collect(), // Convert iterator into a slice of f32 values (0-1 range).
                Err(_) => return Err(std::io::ErrorKind::NotFound),
            }
        } else {
            return Err(std::io::ErrorKind::NotFound);
        }

        Ok(self)
    }

    /// Serializes a buffer pallet to an external crate's file path (e.g., JS/TX/QT/VLC).
    pub fn serialize_to_external<Crates>(self, crates: &mut dyn Crates + 'static) -> Result<String> {
        let mut output = String::new();

        // Write the data directly to a temporary file if needed. This is standard for external asset loading (JS/TX/QT/VLC).
        match crate::loader::TemporaryFileWriter::write(&self.data, &mut File::create("temp_data.bin")) {
            Ok(_) => output = String::from_utf8_lossy(&output), // Convert bytes to UTF-8 string for cross-platform compatibility.
            Err(e) => return Err(std::io::ErrorKind::NotFound),
        }

        crates.write_file_path(output)?;
        
        Ok(output)
    }

    /// Deserializes a buffer pallet from an external crate's file path (e.g., JS/TX/QT/VLC).
    pub fn deserialize_from_external<Crates>(self, crates: &mut dyn Crates + 'static) -> Result<BufferPallet> {
        let mut input = String::new();

        // Read the serialized data from a temporary file. This is standard for external asset loading (JS/TX/QT/VLC).
        match crate::loader::TemporaryFileReader::read(&input, &mut File::create("temp_data.bin"))? {
            Ok(data) => data.into_iter().collect(), // Convert iterator into a slice of f32 values.
            Err(e) => return Err(std::io::ErrorKind::NotFound),
        }

        Ok(self)
    }
}

/// Trait for the core signal processing pipeline that orchestrates phase-aligned cepstral correlation (CEPS).
pub trait SignalProcessor {
    /// Computes CEPS coefficients per sample index, correlating with ripeness/frozen status data.
    fn compute_ceps(&self, ceps_data: &[f32], frozen_status: &Vec
