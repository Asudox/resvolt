#![deny(unsafe_code)]
#![warn(missing_docs, clippy::missing_panics_doc)]

//! # [Resvolt](crate)
//! ## A Rust-based API wrapper for the Revolt chat platform.

#[doc(inline)]
pub use client::*;
mod client;

#[doc(hidden)]
pub mod error;

/// Revolt API Permissions
pub mod api_permissions;
pub mod builders;
#[cfg(feature = "cache")]
pub mod cache;
pub mod http;
pub mod models;
#[cfg(feature = "state")]
pub mod state;

pub(crate) mod websocket;
