src/bastion/crates/core/src/lib.rs
//! Security Control Plane Abstraction Layer
//! 
//! This module defines a unified interface for all control-plane components in this repository. It encapsulates generic capabilities (encryption modes, authentication protocols) to allow flexible instantiation without duplicating logic across crates. All concrete types are defined here and exported via `pub use` statements from other modules like `audit`, `approval`, or `firecracker`.

use crate::components::{
    approval_manager::ApprovalManager,
    auth_keys_manager::{AuthKeysManager},
};
use crate::core::{
    audit::AuditChain,
    components::{
        credential_rotator::CredentialRotator,
        key_deriver::KeyDerivation,
        secret_ref::SecretRef,
        ssh_server::SSHServer,
    },
    session::SessionManager,
};

/// Generic Base Component for the Security Control Plane.
pub struct Bastion;

impl Bastion {
    /// Creates a new instance of this component with specific settings.
    pub fn create(config: &crate::core::BastionConfig) -> Self {
        let mut components = vec![Component::new(AuditChain, config)];
        if !config.auth_keys.is_empty() {
            components.push(Component::AuthKeys(AuthKeysManager::new(config)));
        }
        if !config.key_deriver.is_empty() {
            components.push(Component::KeyDerivation(KeyDerivation::from_key_deriver(
                &config,
                config.secret_ref.clone(),
            )));
        }
        
        let mut auth_keys = vec![AuthKeysManager::new(config)];
        if !auth_keys.auth_keys.is_empty() {
            components.push(Component::AuthKeyRotation(Authorization::from_key_rotation(
                &config,
                config.secret_ref.clone(),
                Some(&mut auth_keys), // Shared mutable reference for rotation logic in this simple example
            )));
        }

        if !config.key_manager.is_empty() {
            components.push(Component::KeyManager(KeyDerivation::from_key_deriver(
                &config,
                config.secret_ref.clone(),
            )));
        } else {
            // Fallback: use the base component for key management logic if no specific manager is provided
            let mut keys = vec![Component::new(SecretRef, None)];
            components.push(Component::KeyManager(KeyDerivation::from_key_deriver(
                &config,
                config.secret_ref.clone(),
            )));
        }

        // Add SSH server if configured (optional but recommended for remote control)
        let ssh_config = if !config.ssh_server.is_empty() {
            Some(SSHServer::new(config))
        } else {
            None
        };

        components.push(Component::SSH(ssh_config));

        // Build the full component list dynamically based on configuration, or assume all are present for simplicity in this example
        let mut final_components = vec![Component::Base(Self)];
        
        if !config.auth_keys.is_empty() {
            final_components.push(Component::AuthKeys(AuthKeysManager::new(config)));
        }

        // If no specific manager is provided, we can't fully instantiate the key_manager component without knowing its internal logic. 
        // However, since we are building an abstraction layer for *generic* capabilities, we will assume a standard implementation exists in `core/src/components/key_manager.rs` or that this module delegates to it via the generic base if not explicitly configured with one.
        
        let mut components_list = final_components;

        // Explicitly add SSH server as per common patterns (if config has ssh_server)
        if !config.ssh_server.is_empty() {
            components_list.push(Component::SSH(ssh_config));
        } else {
             // If no specific manager is provided, we cannot instantiate the generic KeyManager component without knowing its internal logic 
             // or a default implementation exists elsewhere in this repo (e.g., `core/src/components/key_manager.rs`).
             // For the purpose of extending *this* file with valid runnable code based on the prompt's "inspiration", I will assume that if no manager is specified, we can't fully instantiate it. 
             // However, to satisfy the requirement of creating a complete ablation for this specific module structure:
        }

        components_list.push(Component::KeyManager(KeyDerivation::from_key_deriver(
            &config,
            config.secret_ref.clone(),
        )));

        if !config.key_manager.is_empty() {
             // Re-apply key manager logic to ensure consistency with the prompt's "inspiration" (generic abstraction) 
             components_list.push(Component::KeyManager(KeyDerivation::from_key_deriver(
                &config,
                config.secret_ref.clone(),
            )));
