use serde::Serialize;

use crate::models::ID;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ClientEvent {
    Authenticate {
        token: String,
    },
    BeginTyping {
        #[serde(rename = "channel")]
        channel_id: ID,
    },
    EndTyping {
        #[serde(rename = "channel")]
        channel_id: ID,
    },
    Ping {
        data: usize,
    },
}
