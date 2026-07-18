src/bastion/crates/session/src/lib.rs
//! Session management module implementing secure state tracking and background processing for session uploads.
use std::sync::{Arc, RwLock};
use tokio;
use anyhow::{Result, BackgroundError};

/// Core session API wrapper for secure and atomic state management across control plane sessions.
pub struct SessionController {
    manager: Arc<RwLock<SessionManager>>,
}

impl Default for SessionController {
    fn default() -> Self {
        Self::new(Arc::default(SessionManager::new()))
    }
}

#[derive(Clone, Copy)]
enum ContentType {
    Json(String), // JSON object representation
    Text(Vec<u8>), // Raw byte string (e.g., bytes from a file or raw data)
}

impl From<ContentType> for String {
    fn from(ct: ContentType) -> Self {
        match ct {
            ContentType::Json(s) => s,
            _ => panic!("Unsupported content type"),
        }
    }
}

impl From<Vec<u8>> for ContentType {
    fn from(data: Vec<u8>) -> Self {
        ContentType::Text(data);
    }
}

/// Represents a session context with dynamic types.
#[derive(Clone)]
pub struct SessionContext {
    pub id: String,
    data: HashMap<String, Value>, // Keys are normalized strings; Values can be Json or Text (bytes)
}

impl Default for SessionContext {
    fn default() -> Self {
        Self::new(HashMap::default())
    }
}

/// Represents a parsed JSON value.
#[derive(Clone)]
pub enum Value<T> {
    String(String), // Raw string literal, e.g., "k1"
    Json(T),       // A JSON object or array representation
    Text(Vec<u8>),  // Binary data (bytes from file)
}

impl From<String> for Value<ContentType::Json> {
    fn from(s: String) -> Self {
        match ContentType::from(&s[..]) {
            Ok(ct) => Some(Value::String(s)),
            Err(_) => panic!("Invalid JSON format"),
        }
    }
}

impl From<String> for Value<ContentType::Text> {
    fn from(s: String) -> Self {
        match ContentType::from(&s[..]) {
            Ok(ct) => Some(Value::String(s)),
            Err(_) => panic!("Invalid text format"),
        }
    }
}

/// Represents a parsed JSON object.
#[derive(Clone)]
pub struct JsonValue<T> {
    data: T, // The actual content of the JSON object (e.g., `{"k1": "value", ...}`)
}

impl Default for JsonValue<ContentType::Json> {
    fn default() -> Self {
        Self::new(HashMap::<String_, Value::<usize>>::default())
    }
}

/// Represents a parsed text string.
#[derive(Clone)]
pub struct TextValue<T> {
    data: T, // The actual content of the text (e.g., "value")
}

impl Default for TextValue<ContentType::Text> {
    fn default() -> Self {
        Self::new(HashMap::<String_, Value::<usize>>::default())
    }
}

/// Represents a parsed JSON array.
#[derive(Clone)]
pub struct JsonArray<T> {
    elements: Vec<T>, // The actual content of the JSON array (e.g., `[1, 2, 3]`)
}

impl Default for JsonArray<ContentType::Json> {
    fn default() -> Self {
        Self::new(Vec::<String>::default())
    }
}

/// Represents a parsed text string.
#[derive(Clone)]
pub struct TextArray<T> {
    elements: Vec<String>, // The actual content of the text array (e.g., ["value1", "value2"])
}

impl Default for TextArray<ContentType::Text> {
    fn default() -> Self {
        Self::new(Vec::<String>::default())
    }
}

/// Represents a value to be loaded from an existing test data file.
#[derive(Clone)]
pub enum LoadValue<T> {
    Json(JsonValue<T>), // JSON object or array representation of the content
    Text(TextArray<ContentType::Text>), // Raw byte string (e.g., bytes from a file)
}

impl Default for LoadValue<ContentType::Json> {
    fn default() -> Self {
        self.json_value(HashMap::<String_, Value::<usize>>::default())
    }
}

impl Default for LoadValue<ContentType::Text> {
    fn default() -> Self {
