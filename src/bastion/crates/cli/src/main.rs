src/bastion/crates/cli/src/main.rs
```rust
use clap::{Arg, ArgMatches};
use tracing::info;
use std::sync::Arc;
use anyhow::{Result, Context};
use bytes::{BytesMut, ReadExt as BytesReadExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
enum BastionError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field
}

impl BastionError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<BastionError>, message: &str) -> Result<Self> {
        match error_type.into() {
            BastionError::MissingKey(key) => Ok(BastenError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, BastionError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == BastionError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != BastionError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == BastionError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema definition
    pub fn new_schema_definition(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == BastionError::MissingKey(_) && self.is_missing() }

    // Public method to construct the complete schema definition for validation
    pub fn new_schema_definition_with_data(schema_map: HashMap<String, String>, data_values: Vec<&'static str>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == BastionError::MissingKey(_) && self.is_missing() }
}

pub struct Command;

impl Command {
    /// Build the CLI command builder with all required arguments and subcommands.
    #[allow(dead_code)]
    static CONFIG_FILE: &str = "bastion_config.json";
    const DEFAULT_TTL: i64 = 3600;

    pub fn build_cli() -> Command {
        let mut args = Vec::new();
        
        // Configuration Management for Bastion CLI
        #[allow(dead_code)]
        static CONFIG_FILE: &str = "bastion_config.json";
        const DEFAULT_TTL: i64 = 3600;

        Args::add_command(
            Arg::new("config")
                .short('c')
                .long("configuration-file")
                .help("Path to the configuration JSON file (default: {})", default_value!("{}", CONFIG_FILE))
                .arg_required_else_help(true),
        );

        let config_file = match args.get_matches() {
            matches::ArgMatches::None => None,
            _ => Some(args.value_of::<String>("config".to_string())),
        };

        Args::add_subcommand(
            Arg::new("session").about("Create a new session for audit tracking"),
        )
        .subcommand(Args::add_arg(Arg::new("ttl")
            .short('t')
            .long("ttl-seconds") // Default: 3600 (1 hour)
            .default_value(DEFAULT_TTL))
                .help("Time duration in seconds for session creation"),

        )
        .subcommand(Args::add_arg(Arg::new("health").about("Check control-plane health status")))
            .arg_required_else_help(true),

        // Audit Management Functions
        Args::add_subcommand(Args::add_arg(Arg::new("audit-export")
            .short('e')
            .long("export-logs")
            .help("Export current audit logs to JSON file"))
            .subcommand(Args::add_arg(Arg::new("verify").about("Verify the integrity of the log chain")))

        )
