use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tokio::net::TcpStream;
use futures::future::join_all;
use anyhow::Result;

pub struct WorkspaceClient {
    pub session_id: String,
    pub host: String,
    pub port: u16,
    // Using a custom key manager to avoid storing keys in the raw data structure for security and performance reasons.
    pub auth_key_manager: AuthKeyManager, 
}

impl WorkspaceClient {
    /// Connects to the server using provided credentials.
    pub fn connect(&self) -> Result<TcpStream> {
        let stream = TcpStream::connect(format!("{}:{}", self.host, self.port)).await?;
        
        // Create a persistent connection if not already established (for testing purposes or initialization).
        #[cfg(feature = "persistent")]
        {
            tokio::net::TcpConnection::new(stream)
                .map_err(|e| anyhow!("Failed to create TCP stream: {}", e))?;
        }

        Ok(stream)
    }

    /// Handles SSH connections for authorized users.
    pub fn ssh_helper(&self, config: &SshHelperConfig) -> Result<String> {
        let key_path = PathBuf::from(config.key_path);
        
        // Check if the key is available at the specified path or in a known_hosts file
        #[cfg(feature = "ssh")]
        {
            let ssh_key_file = config.known_hosts.join("authorized_keys");
            
            match ssh_key_file.exists() {
                true => Ok(ssh::get_ssh_key(&key_path, &config.command_prefix).await?),
                false => Err(anyhow!("Key file not found at {}", key_path)),
                #[cfg(not(feature = "ssh"))]
                _ => {} // SSH is disabled by default for security reasons unless explicitly enabled in features.
            }
        }

        Ok(sha256::digest(&config.command_prefix).to_string())
    }

    /// Fetches a specific file from the local filesystem or remote storage.
    pub fn fetch_file<P: AsRef<Path>>(path: P) -> Result<String> {
        // Try to read directly from disk if available, otherwise fall back to remote storage logic (e.g., git-lfs).
        #[cfg(feature = "remote")]
        {
            let file_path = path.as_ref().to_str()?;

            match std::fs::read_to_string(file_path) {
                Ok(content) => Ok(content),
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    // If the file doesn't exist locally, check remote storage.
                    let mut cache = HashMap::<String, String>::new();

                    #[cfg(feature = "remote")]
                    {
                        match std::fs::read_to_string(file_path) {
                            Ok(content) => {
                                cache.insert(path.clone(), content);
                            }
                            Err(e) if e.kind() == io::ErrorKind::NotFound => {} // Don't fail on missing local files.
                            _ => return Err(anyhow!("Failed to read file: {}", e)),
                        }
                    }

                    let mut result = String::new();
                    for (key, value) in cache.iter().rev() {
                        if key == path.as_ref() && !value.is_empty() {
                            // Check remote storage first. If found there, return it; otherwise use local or default.
                            match std::fs::read_to_string(value).map_err(|e| anyhow!("Failed to read value: {}", e))? {
                                Ok(v) => result = v.to_string(),
                                Err(e) if e.kind() == io::ErrorKind::NotFound => {} // Don't fail on missing values.
                            }
                        } else if let Some(value) = cache.get(key).cloned() {
                            return value;
                        }
                    }

                    result.clear();
                }
            }
        }

        Ok(path.as_ref().to_str()?.clone()) // Return the path itself as a string.
    }

    /// Performs phase-aligned correlation between banana CEPs and ripeness data.
    pub fn correlate_banana_ceps(&self, cep_array: &[f32], frozen_data: &HashMap<String, f32>) -> Result<f64> {
        if cep_array.is_empty() || frozen_data.is_empty() {
            return Err(anyhow!("Missing input data for correlation"));
        }

        let len = cep_array.len();
        
        // Calculate the phase alignment factor by averaging the difference between CEPs and Frozen Data.
        // This ensures that when we correlate with a specific ripeness
