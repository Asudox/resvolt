use serde::{Deserialize, Serialize};

use crate::models::ID;

/// Server system message channels.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SystemMessageChannels {
    /// User joined channel id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_joined: Option<ID>,
    /// User left channel id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_left: Option<ID>,
    /// User kicked channel id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_kicked: Option<ID>,
    /// User banned channel id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_banned: Option<ID>,
}

impl SystemMessageChannels {
    /// Set the user joined channel.
    pub fn user_joined(mut self, id: &ID) -> Self {
        self.user_joined = Some(id.clone());
        self
    }

    /// Set the user left channel.
    pub fn user_left(mut self, id: &ID) -> Self {
        self.user_left = Some(id.clone());
        self
    }

    /// Set the user kicked channel.
    pub fn user_kicked(mut self, id: &ID) -> Self {
        self.user_kicked = Some(id.clone());
        self
    }

    /// Set the user banned channel.
    pub fn user_banned(mut self, id: &ID) -> Self {
        self.user_banned = Some(id.clone());
        self
    }
}
