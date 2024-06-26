use serde::Serialize;

use crate::{
    builders::CreateEmbed,
    models::{Masquerade, ID},
};

/// Builder for create a message.
#[derive(Debug, Clone, Serialize)]
pub struct CreateMessage {
    content: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<ID>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    replies: Vec<Reply>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    embeds: Vec<CreateEmbed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    masquerade: Option<Masquerade>,
}

#[derive(Debug, Clone, Serialize)]
struct Reply {
    id: ID,
    mention: bool,
}

impl CreateMessage {
    /// Creates a new builder.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            attachments: Vec::new(),
            replies: Vec::new(),
            embeds: Vec::new(),
            masquerade: None,
        }
    }

    /// Set a attachment to include in the message.
    pub fn attachment(mut self, id: &ID) -> Self {
        self.attachments.push(id.clone());
        self
    }

    /// Set a message to reply to.
    pub fn reply(mut self, id: &ID, mention: bool) -> Self {
        self.replies.push(Reply {
            id: id.clone(),
            mention,
        });
        self
    }

    /// Set a embed to include in the message.
    pub fn embed(mut self, build: impl Fn(CreateEmbed) -> CreateEmbed) -> Self {
        self.embeds.push(build(CreateEmbed::default()));
        self
    }

    /// Set the masquerade.
    pub fn masquerade(mut self, masquerade: Masquerade) -> Self {
        self.masquerade = Some(masquerade);
        self
    }
}

impl<T: Into<String>> From<T> for CreateMessage {
    fn from(content: T) -> Self {
        Self::new(content)
    }
}

/// Builder for edit a message.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EditMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    embeds: Vec<CreateEmbed>,
}

impl EditMessage {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set a embed to include in the message.
    pub fn embed(mut self, build: impl Fn(CreateEmbed) -> CreateEmbed) -> Self {
        self.embeds.push(build(CreateEmbed::default()));
        self
    }
}

impl<T: Into<String>> From<T> for EditMessage {
    fn from(content: T) -> Self {
        Self::new().content(content)
    }
}
