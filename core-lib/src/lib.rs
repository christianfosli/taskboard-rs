use serde::{Deserialize, Serialize};

mod project;
pub use crate::project::*;
pub use uuid;

mod task;
pub use crate::task::*;

/// Commands to serialize and sent to back-end
pub mod commands;

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Errors from our back-end API should be in this format
pub struct ErrorMessage {
    pub message: String,
    pub status_code: u16,
}
