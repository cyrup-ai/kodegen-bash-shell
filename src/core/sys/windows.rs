pub use super::stubs::commands;
pub use super::stubs::fd;
pub use super::stubs::fs;
pub use super::stubs::input;
pub(crate) mod network;
pub use super::stubs::resource;

/// Signal processing utilities
pub mod signal {
    pub(crate) use super::super::stubs::signal::*;
    pub(crate) use tokio::signal::ctrl_c as await_ctrl_c;
}

pub use super::stubs::terminal;
pub use super::tokio_process as process;
pub(crate) mod users;

/// Platform-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum PlatformError {}
