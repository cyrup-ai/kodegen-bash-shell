//! kodegen-bash-shell: Embeddable POSIX/bash shell with cancellation support.
//!
//! This is a permanent fork of [brush-shell](https://github.com/reubeno/brush) with
//! programmatic command cancellation support via `CancellationToken`.

pub mod builtins;
pub mod core;
pub mod interactive;
pub mod parser;
pub mod shell;

// Re-export CancellationToken for users
pub use tokio_util::sync::CancellationToken;

// Re-export commonly used types from core at crate root
pub use core::{
    CommandArg, CreateOptions, Error, ErrorKind, ExecutionContext, ExecutionControlFlow,
    ExecutionExitCode, ExecutionParameters, ExecutionResult, ExecutionSpawnResult,
    ProcessGroupPolicy, Shell, ShellBuilder, ShellBuilderState, ShellFd, ShellValue,
    ShellVariable, BuiltinError,
};

// Re-export parser types
pub use parser::{
    ParseError, Parser, ParserBuilder, ParserOptions, SourceInfo, Token, TokenLocation,
    TokenizerError, TokenizerOptions,
};

// Re-export builtins types
pub use builtins::{BuiltinSet, ShellBuilderExt, default_builtins};
