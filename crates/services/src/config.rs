use chrono::{self, DateTime, Local};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use std::{fmt::Display, path::PathBuf};

/// # Servers
/// This restricts user's query
/// Write servers you use below
#[derive(Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Servers {
    Auth,
    Handler,
    Videos,
    ChatBot,
}
/// Modify here to show, save server with specific name.
impl AsRef<str> for Servers {
    fn as_ref(&self) -> &str {
        match self {
            Self::Auth => "Auth",
            Self::Handler => "Handler",
            Self::Videos => "Videos",
            Self::ChatBot => "Chatbot",
        }
    }
}

/// Save duration for logging level (in hours)
#[repr(usize)]
pub(crate) enum Expiration {
    Debug = 24,
    Info = 24 * 2,
    Warn = 24 * 7,
    Error = 24 * 14,
}

/// Converts logging level to file path.
/// Changing this mapping will alter where logs are saved.
impl From<Level> for PathBuf {
    fn from(value: Level) -> Self {
        let mut path = Self::from("./logs"); // base path
        match value {
            Level::Debug => path.push("debug"),
            Level::Info => path.push("info"),
            Level::Warn => path.push("warn"),
            Level::Error => path.push("error"),
        };
        path
    }
}

/// Logging levels, Don't implement
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct LogQuery {
    pub service: Servers,
    pub message: String,
    pub trace_id: String,
}

pub(crate) struct LogEntry {
    time: DateTime<Local>,
}

struct DateTimeExt(DateTime<Local>);
impl Serialize for DateTimeExt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("LogEntry", 5)?;
        s.serialize_field("time", &self.0.to_string())?;
        s.end()
    }
}

impl Display for Servers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self)
    }
}
