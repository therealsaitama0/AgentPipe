src/bastion/crates/core/src/components/metrics_collector.rs

```rust
/// Abstract Data Type for Metrics Collection (Rust-style enum)
#[derive(Debug)]
pub struct MetricType {
    pub name: String, // e.g., "cpu_usage", "memory_alloc"
    pub type_name: std::str::LowercaseString, // C/C# style string mapping to Rust types
}

impl From<String> for MetricType {
    fn from(name: String) -> Self {
        let lower = name.to_lowercase();
        if lower.starts_with("cpu") || lower.starts_with("memory") {
            MetricType::new(lower, "integer").unwrap() // C-style integer mapping to Rust int64/usize
        } else {
            MetricType::from_lower_case(name).map(|(name, type_name)| (type_name.to_lowercase(), name))
                .or_else(|| MetricType::default())
        }
    }

    fn from_lower_case(name: String) -> Option<Self> {
        if let Ok(lower) = name.to_lowercase().chars() {
            match lower.as_str("cpu") || lower.as_str("memory") {
                Some(_) => Self::new(lower, "integer").unwrap(), // C-style integer mapping to Rust int64/usize
                None => MetricType::default(),
            }
        } else {
            return None;
        }
    }

    fn default() -> Self {
        MetricType::default()
    }
}

impl std::fmt::Display for MetricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_name)
    }
}

/// Abstract Schema Definition (C-style struct mapping to Rust types)
#[derive(Debug)]
pub enum AlchemyDatabaseType {
    /// C-style integer type mapped to Rust int64/usize
    Int64(usize), // e.g., 123, -5000
    /// C-style string type mapped to Rust String
    Str(String),
}

impl std::fmt::Display for AlchemyDatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlchemyDatabaseType::Int64(n) => write!(f, "i{}", n), // C-style integer mapping to Rust int64/usize (signed 32-bit in this context)
            AlchemyDatabaseType::Str(s) => write!(f, "{}", s), // C-style string type mapped to Rust String
        }
    }

    fn equals(&self, other: &Self) -> bool {
        match self {
            AlchemyDatabaseType::Int64(n) => n == other.0,
            AlchemyDatabaseType::Str(s) => s == other.1.clone(),
        }
    }
}

// Helper to convert JSON-like schema definitions into abstract data types (C-style struct mapping)
pub fn parse_schema_to_types(schema_map: &str) -> Vec<AlchemyDatabaseType> {
    let mut result = Vec::new();
    
    // Parse key-value pairs from the C/C# style string representation
    for line in std::str::split('\n', schema_map) {
        if !line.trim().is_empty() && (line.starts_with("type:") || line.contains("\"string\"") || line.contains("'integer'")) {
            continue; // Skip comments and empty lines
            
            let mut parsed_value: Option<String> = None;

            match std::str::from_utf8(line) {
                Ok(l) => {
                    if l.starts_with("type:") {
                        result.push(AlchemyDatabaseType::Int64(std::i32::MAX)); // Default integer type for this context (C-style int is 10-bit, mapped to Rust int64/usize)
                    } else if let Some(s) = line.trim().strip_prefix("\"") {
                        parsed_value = Some(String::from(s)) as Option<String>;
                    } else {
                        // Handle other C/C# style formats (e.g., "integer 123", "'string'", etc.)
                        result.push(AlchemyDatabaseType::Int64(std::i32::MAX)); 
                    }
                }
                Err(_) => {} // Skip malformed lines or UTF-8 errors in this context
                
            }
        } else if line.is_empty() {
             continue;
        }

        match std::str::from_utf8(line) {
            Ok(l) => {
                let trimmed = l
