#![allow(dead_code)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::unused_async)]
#![allow(clippy::unused_self)]

// Re-export modules from crate::core for use by stub implementations
pub use crate::core::error;
pub use crate::core::interfaces;
pub use crate::core::openfiles;
pub use crate::core::sys;
pub use crate::core::traps;
pub use crate::ShellFd;

pub mod commands;
pub mod fd;
pub mod fs;
pub mod input;
pub(crate) mod network;
pub(crate) mod pipes;
pub mod process;
pub mod resource;
pub mod signal;
pub mod terminal;
pub(crate) mod users;
