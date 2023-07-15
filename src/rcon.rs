use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::rcon_web::WebRconMessage;

#[async_trait]
pub trait Rcon {
    async fn connect(ip: &str, port: &str, password: &str) -> Self;
    async fn execute(&mut self, command: &str);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(non_snake_case)]
pub enum RconMessage {
    Generic {
        message: String,
    },
    Warning {
        message: String,
    },
    Error {
        message: String,
    },
    Chat {
        Channel: u64,
        Message: String,
        UserId: String,
        Username: String,
        Color: String,
        Time: u64,
    },
    Disconnected,
}

impl RconMessage {
    pub fn from_webrcon_message(message: WebRconMessage) -> Self {
        match &*message.Type {
            "Chat" => serde_json::from_str(&message.Message).unwrap(),
            "Generic" => RconMessage::Generic {
                message: message.Message,
            },
            "Warning" => RconMessage::Warning {
                message: message.Message,
            },
            "Error" => RconMessage::Error {
                message: message.Message,
            },
            _ => panic!("Cannot parse WebRconMessage. {}", message.Type),
        }
    }
}
