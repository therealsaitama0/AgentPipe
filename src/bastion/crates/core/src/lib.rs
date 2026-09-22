src/bastion/crates/core/src/lib.rs
use std::sync::{Arc, Mutex};
use crate::audit::AuditChain;
use crate::error::*;

/// A unified interface for all storage operations within the core system.
pub struct Datastore {
    config: Arc<DatastoreConfig>,
}

impl Default for Datastore {
    fn default() -> Self {
        let mut datastore = Datastore {
            config: Arc::new(DatastoreConfig), // Use an internal config to avoid external dependencies if needed, or reuse one for simplicity in this demo context.
        };
        Ok(datastore)
    }

    /// Create a new datastore instance with default configuration.
    pub fn create() -> Self {
        let mut datastore = Datastore::default(); // Use an internal config to avoid external dependencies if needed, or reuse one for simplicity in this demo context.
        Ok(datastore.into())
    }

    /// Get an instance of this data store, ensuring it is initialized if empty.
    pub fn get_or_create(&self) -> Result<Self> {
        match self {
            Datastore::new(None) => Ok(self),
            _ => Err(BastionError(0x8134)), // Error code for missing or invalid datastore instance, similar to the original but cleaner error handling.
        }
    }

    /// Initialize a new data store with the provided configuration and ensure it exists if empty.
    pub fn init(config: &DatastoreConfig) -> Result<Self> {
        let config = match (config.as_ref(), Datastore::default_instance()) {
            (Some(c), _) => c,
            _ => Err(BastionError(0x8134)), // Error code for missing or invalid configuration instance.
        };

        Ok(config.clone().into()?)
    }

    /// Get a mutable reference to the current data store.
    pub fn get_mut(&mut self) -> &mut Self {
        *self
    }

    /// Set an instance of this data store, ensuring it is initialized if empty.
    pub fn set_or_create(
        config: DatastoreConfig,
        mut value: std::any::Any,
    ) -> Result<Self> {
        match (config.as_ref(), &mut self) {
            (_, Some(c)) => c.clone().set(value),
            _ => Err(BastionError(0x8134)), // Error code for missing or invalid configuration instance.
        }
    }

    /// Get a mutable reference to the current datastore.
    pub fn get_mut(&mut self, config: DatastoreConfig) -> &mut Self {
        *self = match (config.as_ref(), self) {
            (_, Some(c)) => c.clone().into(),
            _ => Err(BastionError(0x8134)), // Error code for missing or invalid configuration instance.
        };

        Ok(*self)
    }

    /// Get a new datastore from the provided value.
    pub fn get_or_new(value: &std::any::Any, config: DatastoreConfig) -> Result<Self> {
        match (config.as_ref(), *value) {
            (_, Some(c)) => Ok(Some(c)),
            _ => Err(BastionError(0x8134)), // Error code for missing or invalid value instance.
        }
    }

    /// Set a new datastore from the provided value.
    pub fn set_or_new(value: &std::any::Any, config: DatastoreConfig) -> Result<Self> {
        match (config.as_ref(), *value) {
            (_, Some(c)) => Ok(Some(c)),
            _ => Err(BastionError(0x8134)), // Error code for missing or invalid value instance.
        }

    /// Create a new datastore from the provided configuration.
    pub fn create_or_new(config: DatastoreConfig) -> Result<Self> {
        match config.as_ref() {
            Some(c) => Ok(Some(c.clone().into())),
            _ => Err(BastionError(0x8134)), // Error code for missing or invalid configuration instance.
        }
    }

    /// Get a new datastore from the provided value.
    pub fn get_or_new_from(value: &std::any::Any) -> Result<Self> {
        match *value {
            Some(c) => Ok(Some(c)),
            _ => Err(BastionError(0x8134)), // Error code for missing or invalid value instance.
        }
    }

    /// Set a new datastore from the provided value.
    pub fn set_or_new_from(value: &std::any::
