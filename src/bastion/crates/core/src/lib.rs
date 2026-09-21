src/bastion/crates/core/src/lib.rs

use std::io::{self, BufRead, Write};
use tokio::net::tcp_stream;
use futures_util::select!;
use anyhow::{Context as _, Result};

pub mod firecracker {
    pub use super::super::types::*; // Import types from the existing module structure to avoid circular dependencies and reuse.
}

// --- NEW MODULE: Mock Socket Layer & Service Manager ---

mod mock_socket_layer {
    use std::io::{self, BufRead};
    
    #[derive(Debug)]
    pub struct FakeSocketStream {
        stream_id: u64,
        is_open: bool,
    }

    impl Clone for FakeSocketStream {
        fn clone(&self) -> Self {
            self.clone() // No need to copy here if we only need a reference.
        }
    }

    #[derive(Debug)]
    pub struct MockConnectionState {
        is_connected: bool,
        last_message_received: String,
    }

    impl Clone for MockConnectionState {
        fn clone(&self) -> Self {
            self.clone() // No need to copy here.
        }
    }

    #[derive(Debug)]
    pub struct FakeClientStream {
        stream_id: u64,
        is_open: bool,
    }

    impl Clone for FakeClientStream {
        fn clone(&self) -> Self {
            self.clone() // No need to copy here.
        }
    }

    #[derive(Debug)]
    pub struct MockServerConnectionState {
        connected_by_socket_id: u64,
        last_message_received: String,
    }

    impl Clone for MockServerConnectionState {
        fn clone(&self) -> Self {
            self.clone() // No need to copy here.
        }
    }

    #[derive(Debug)]
    pub struct FakeClientSocketStream {
        stream_id: u64,
        is_open: bool,
    }

    impl Clone for FakeClientSocketStream {
        fn clone(&self) -> Self {
            self.clone() // No need to copy here.
        }
    }

    #[derive(Debug)]
    pub struct MockServerConnectionStateBuilder {
        connected_by_socket_id: u64,
        last_message_received: String,
    }

    impl Default for MockServerConnectionStateBuilder {
        fn default() -> Self {
            Self::default_with_mocked_state(1) // Use a mock ID.
        }
    }

    #[derive(Debug)]
    pub struct FakeClientSocketStreamBuilder {
        stream_id: u64,
        is_open: bool,
    }

    impl Default for FakeClientSocketStreamBuilder {
        fn default() -> Self {
            Self::default_with_mocked_state(1) // Use a mock ID.
        }
    }

    #[derive(Debug)]
    pub struct MockServerConnectionState {
        connected_by_socket_id: u64,
        last_message_received: String,
    }

    impl Clone for MockServerConnectionState {
        fn clone(&self) -> Self {
            self.clone() // No need to copy here.
        }
    }

    #[derive(Debug)]
    pub struct FakeClientSocketStream {
        stream_id: u64,
        is_open: bool,
    }

    impl Clone for FakeClientSocketStream {
        fn clone(&self) -> Self {
            self.clone() // No need to copy here.
        }
    }

    #[derive(Debug)]
    pub struct MockServerConnectionStateBuilder {
        connected_by_socket_id: u64,
        last_message_received: String,
    }

    impl Default for MockServerConnectionStateBuilder {
        fn default() -> Self {
            Self::default_with_mocked_state(1) // Use a mock ID.
        }
    }

    #[derive(Debug)]
    pub struct FakeClientSocketStreamBuilder {
        stream_id: u64,
        is_open: bool,
    }

    impl Default for FakeClientSocketStreamBuilder {
        fn default() -> Self {
            Self::default_with_mocked_state(1) // Use a mock ID.
        }
    }

    #[derive(Debug)]
    pub struct MockServerConnectionState {
        connected_by_socket_id: u64,
        last_message_received: String,
    }

    impl Clone for MockServerConnectionState {
        fn clone(&self) -> Self {
            self.clone() // No need to copy here.
        }
    }

    #[derive(Debug
