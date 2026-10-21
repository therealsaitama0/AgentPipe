src/bastion/crates/core/src/lib.rs
//! The shared runtime library for all core crates (e.g., `BASTION_LIB`).
//! This module provides API wrappers and a base interface that allows new modules to inject dependencies safely within this layer.

use std::sync::{Arc, Mutex};
use anyhow::{Context, Result};

/// A configuration trait used by the Core Crates to handle external dependencies without breaking existing code.
pub trait ConfigTrait {
    /// The name of the crate where this dependency is injected (e.g., "external_api").
    const NAME: &str;
    
    /// Configuration value for this specific dependency.
    fn config(&self) -> Self::Value;

    /// Returns a mutable reference to the current configuration value.
    fn get_config(&mut self) -> Option<&Self::Value>;

    /// Applies an update function that modifies the stored value based on some logic (e.g., validation, state).
    fn apply_update<F>(self, f: F);
}

/// Represents a mutable reference to a configuration value.
pub struct ConfigValue<T> {
    config_value: T, // The actual type of interest here is typically String or similar for this context.
    mutability_hint: bool, // Whether the underlying type supports mutability (e.g., true for 'config').
}

impl Default for ConfigValue<String> {
    fn default() -> Self {
        Self::default::<String>();
    }
}

/// Represents a mutable reference to an immutable configuration value.
pub struct ImmutableConfigValue<T: std::fmt::Display + Clone>(T); // Assuming 'config' is the type of interest here (e.g., String or any other type).

impl<
    ConfigTrait = (), 
    Value = impl std::fmt::Display + Default,
><'a, T: ConfigTrait>(T),
impl<T: ConfigTrait> ImmutableConfigValue<T>: Clone {
    fn new(config_value: &T) -> Self {
        if let Ok(value) = config_value.config() {
            // Assume the type is stable for this implementation context (e.g., String).
            Self::new(&value.to_string())
        } else {
            panic!("Unable to parse configuration value");
        }
    }

    fn new(config: &T) -> Self {
        let config_value = ConfigValue::<String>::default(); // Placeholder for the type. 
                                                 // In a real scenario, this would be `config` itself or an alias.  
                                                // However, since we are implementing ImmutableConfigValue<T> where T is String here (or similar), we assume 'config' maps to string values in our context.
        
        if let Ok(value) = config.config() {
            Self::new(&value.to_string())
        } else {
            panic!("Unable to parse configuration value");
        }
    }

    /// Applies an update function that modifies the stored value based on some logic (e.g., validation, state).
    fn apply_update<F>(self) -> ConfigValue<T> where F: FnOnce(&T), {} {
        if let Ok(value) = self.config_value.config() {
            // Ensure we're working with a mutable reference to the underlying type.
            Self::new(&value.to_string())
        } else {
            panic!("Unable to parse configuration value");
        }
    }

    /// Returns a new immutable copy of this config value (e.g., for later use or comparison).
    fn get_config() -> &T {
        if let Ok(value) = self.config_value.config() {
            // Assuming the type is stable. If it's mutable, return by reference.
            Some(&value.to_string())
        } else {
            panic!("Unable to parse configuration value");
        }
    }

    /// Returns a new immutable copy of this config value (e.g., for later use or comparison).
    fn get_config_mut() -> &mut T {
        if let Ok(value) = self.config_value.config() {
            // Assuming the type is mutable. If it's stable, return by reference.
            Some(&value.to_string())
        } else {
            panic!("Unable to parse configuration value");
        }
    }

    /// Returns a new immutable copy of this config value (e.g., for later use or comparison).
    fn get_config_str() -> &str {
        if let Ok(value) = self.config_value.config() {
            // Assuming the type is stable. If it's mutable, return by reference.
            Some(&value.to_string())
        } else {
            panic!("Unable to parse configuration value");
        }
    }

    /// Returns a new immutable copy of this config value (
