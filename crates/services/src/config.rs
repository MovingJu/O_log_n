use std::fmt::Display;

use schemars::JsonSchema;
use serde::Deserialize;

/// # Servers
/// This restricts user's query
#[derive(Deserialize, Clone, JsonSchema)]
#[serde(rename_all="lowercase")]
pub enum Servers {
    Auth,
    Handler,
    Videos,
    ChatBot
}
impl Display for Servers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auth => write!(f, "Auth"),
            Self::Handler => write!(f, "Handler"),
            Self::Videos => write!(f, "Videos"),
            Self::ChatBot => write!(f, "Chatbot")
        }
    }
}