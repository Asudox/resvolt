use serde::{Deserialize, Serialize};

use crate::models::ID;

/// A server category.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Category {
    /// Category id.
    pub id: ID,
    /// Category title.
    pub title: String,
    /// Category channels ids.
    pub channels: Vec<ID>,
}

impl Category {
    /// Creates a new [Category].
    pub fn new(id: ID, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            channels: Vec::new(),
        }
    }
}
