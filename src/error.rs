//! Module for [enum@Error] and [Result] types.

use {
    crate::models::RevoltPermission, reqwest::Error as HttpError, serde::Deserialize,
    thiserror::Error, tokio_tungstenite::tungstenite::Error as WsError,
};

/// Errors that can happen when using [resvolt](crate).
#[derive(Error, Debug)]
pub enum RSError {
    /// Command not found in the client's internal command registry
    #[error("Command not found: {0}")]
    CommandNotFound(String),
    /// Http requests error.
    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),
    /// WebSocket error.
    #[error("WebSocket error: {0}")]
    Ws(#[from] WsError),
    /// Could not authenticate due to an error.
    #[error("Authentication error: {0}")]
    Authentication(#[from] AuthenticationError),
    /// Unknown or unexpected error.
    #[error("Unknown error: {0}")]
    Unknown(String),
    /// No error handler is set, ignore error and continue
    #[error("Error handler not set")]
    ErrorHandlerNotSet,
}

/// Authentication error.
#[derive(Error, Debug, Deserialize, Clone, Copy, PartialEq)]
#[error("{self:?}")]
pub enum AuthenticationError {
    /// Uncategorized error.
    LabelMe,
    /// The Revolt server ran into an issue.
    InternalError,
    /// The token provided is incorrect.
    InvalidSession,
    /// The bot is already authenticated.
    AlreadyAuthenticated,
}

#[allow(missing_docs)]
#[derive(Debug, Deserialize, Error)]
#[serde(tag = "type")]
#[error("{self:?}")]
pub enum APIError {
    AlreadyOnboarded,
    UsernameTaken,
    InvalidUsername,
    UnknownUser,
    AlreadyFriends,
    AlreadySentRequest,
    Blocked,
    BlockedByOther,
    NotFriends,
    UnknownChannel,
    UnknownAttachment,
    UnknownMessage,
    CannotEditMessage,
    CannotJoinCall,
    TooManyAttachments {
        max: u32,
    },
    TooManyReplies {
        max: u32,
    },
    TooManyChannels {
        max: u32,
    },
    TooManyEmbeds {
        max: u32,
    },
    EmptyMessage,
    PayloadTooLarge,
    CannotRemoveYourself,
    GroupTooLarge {
        max: u32,
    },
    AlreadyInGroup,
    NotInGroup,
    UnknownServer,
    InvalidRole,
    Banned,
    TooManyServers {
        max: u32,
    },
    TooManyEmoji {
        max: u32,
    },
    TooManyRoles {
        max: u32,
    },
    ReachedMaximumBots,
    IsBot,
    BotIsPrivate,
    CannotReportYourself,
    MissingPermission {
        permission: RevoltPermission,
    },
    MissingUserPermission {
        permission: String,
    },
    NotElevated,
    NotPrivileged,
    CannotGiveMissingPermissions,
    NotOwner,
    DatabaseError {
        operation: String,
        with: String,
    },
    InternalError,
    InvalidOperation,
    InvalidCredentials,
    InvalidProperty,
    InvalidSession,
    DuplicateNonce,
    VosoUnavailable,
    NotFound,
    NoEffect,
    FailedValidation,
    /// Unknown error
    LabelMe,
}
