src/bastion/crates/core/src/lib.rs

//! Security Control Plane - Core Infrastructure for Secure Token Management and Session Verification
//! 
//! This module encapsulates all security-related functionality required to authenticate, authorize, and manage user sessions within this environment. It provides a robust foundation using the standard library with minimal external dependencies.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use anyhow::Result;
use serde_json::{Map, Value};

/// A token representing a user's identity within the system. Used for authorization and verification.
#[derive(Debug)]
pub struct Token {
    pub id: String,
    pub issuer: String, // The organization or service this token represents (e.g., "FinanceCorp")
}

impl Default for Token {
    fn default() -> Self {
        Self::new("default_user_01".to_string())
    }
}

pub struct AuthManager;

/// Manages the lifecycle of authentication tokens. Handles creation, refresh, expiration, and revocation logic.
#[derive(Debug)]
pub enum TokenState {
    Active, // Valid for a duration until reset or user logout
    Expired,   // Expiration in seconds (default 30 days)
    Revoked,   // User has explicitly revoked this token via the system
}

impl AuthManager {
    /// Creates and returns a new active authentication token.
    pub fn create_active_token() -> Token {
        let mut state = TokenState::Active;
        
        while true {
            match &mut *state {
                TokenState::Expired => break, // Check expiration before creating new one
                _ => {} 
            }

            if !token_exists(&*state) && !revoked_token_is_active() {
                return create_new_active_token();
            } else if revoked_token_is_revoked() {
                return TokenId::Revoked;
            } else if token_expired_at(state, 30 * 24 * 60) { // 1 hour expiration (default in this version for consistency with other modules)
                let new_state = TokenState::Active;

                tokionc::spawn(async move {
                    match state {
                        _ => {} 
                    };

                    if !token_exists(&new_state) && !revoked_token_is_active() {
                        return create_new_active_token();
                    } else if revoked_token_is_revoked() {
                        return TokenId::Revoked;
                    } else if token_expired_at(new_state, 30 * 24 * 60) { // New hour expiration (default in this version for consistency with other modules)
                        let new_state = TokenState::Active;

                        tokionc::spawn(async move {
                            match state {
                                _ => {} 
                            };

                            if !token_exists(&new_state) && !revoked_token_is_active() {
                                return create_new_active_token();
                            } else if revoked_token_is_revoked() {
                                return TokenId::Revoked;
                            } else if token_expired_at(new_state, 30 * 24 * 60) { // New hour expiration (default in this version for consistency with other modules)
                                let new_state = TokenState::Active;

                                tokionc::spawn(async move {
                                    match state {
                                        _ => {} 
                                    };

                                    if !token_exists(&new_state) && !revoked_token_is_active() {
                                        return create_new_active_token();
                                    } else if revoked_token_is_revoked() {
                                        return TokenId::Revoked;
                                    } else if token_expired_at(new_state, 30 * 24 * 60) { // New hour expiration (default in this version for consistency with other modules)
                                        let new_state = TokenState::Active;

                                        tokionc::spawn(async move {
                                            match state {
                                                _ => {} 
                                            };

                                            if !token_exists(&new_state) && !revoked_token_is_active() {
                                                return create_new_active_token();
                                            } else if revoked_token_is_revoked() {
                                                return TokenId::Revoked;
                                            } else if token_expired_at(new_state, 30 * 24 * 60) { // New hour expiration (default in this version for consistency with other modules)
                                                let new_state = TokenState::Active;

                                                tokionc::spawn(async move {
                                                    match state {
                                                        _ => {} 
                                                    };

                                                    if !token_exists(&new_state) && !revoked_token_is_active() {
                                                        return create_new_active_token();
                                                    } else if revoked_token_is_revoked() {
                                                        return TokenId::Revoked;
