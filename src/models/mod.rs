//! Revolt API models.

#[doc(inline)]
pub use {attachment::*, channel::*, message::*, server::*, user::*};

mod attachment;
mod channel;
pub mod events;
mod message;
mod server;
mod user;

/// Models id type.
pub type ID = String;
