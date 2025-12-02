pub use super::sys::stubs::commands;
pub use super::sys::stubs::fd;
pub use super::sys::stubs::fs;
pub use super::sys::stubs::input;
pub(crate) mod network;
pub use super::sys::stubs::resource;

/// Signal processing utilities
pub mod signal {
    pub(crate) use super::sys::stubs::signal::*;
    pub(crate) use tokio::signal::ctrl_c as await_ctrl_c;
}

pub use super::sys::stubs::terminal;
pub use super::sys::tokio_process as process;
pub(crate) mod users;

/// Platform-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum PlatformError {}
