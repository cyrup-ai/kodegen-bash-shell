//! kodegen-bash-shell: Embeddable POSIX/bash shell with cancellation support.
//!
//! This is a permanent fork of [brush-shell](https://github.com/reubeno/brush) with
//! programmatic command cancellation support via `CancellationToken`.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use kodegen_bash_shell::{Shell, ExecutionResult};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut shell = Shell::builder().build().await?;
//!     let result = shell.run_string("echo 'Hello!'", &shell.default_exec_params()).await?;
//!     Ok(())
//! }
//! ```
//!
//! # Custom Builtins
//!
//! See the `custom-builtin` example for how to implement custom shell builtins.

pub mod builtins;
pub mod core;
pub mod interactive;
pub mod parser;
pub mod shell;

// ============================================================================
// Prelude - commonly used types for building shells and custom builtins
// ============================================================================

/// Prelude module for convenient glob imports.
///
/// ```rust
/// use kodegen_bash_shell::prelude::*;
/// ```
pub mod prelude {
    // Core shell types
    pub use crate::core::{
        CommandArg, CreateOptions, Error, ErrorKind, ExecutionContext, ExecutionControlFlow,
        ExecutionExitCode, ExecutionParameters, ExecutionResult, ExecutionSpawnResult,
        ProcessGroupPolicy, Shell, ShellBuilder, ShellBuilderState, ShellFd, ShellValue,
        ShellVariable, BuiltinError,
    };

    // Builtin command infrastructure
    pub use crate::core::builtins::{
        builtin, decl_builtin, simple_builtin, Command, ContentType, DeclarationCommand,
        Registration, SimpleCommand,
    };

    // Builtin registration
    pub use crate::builtins::{BuiltinSet, ShellBuilderExt, default_builtins};

    // Open files / file descriptors
    pub use crate::core::openfiles::{self, OpenFile, OpenFiles};

    // Parser types
    pub use crate::parser::{
        ParseError, Parser, ParserBuilder, ParserOptions, SourceInfo, Token, TokenLocation,
        TokenizerError, TokenizerOptions,
    };

    // Cancellation support
    pub use tokio_util::sync::CancellationToken;

    // Type alias for convenience (matches ExitCode usage in kodegen-tools-terminal)
    pub type ExitCode = ExecutionExitCode;
}

// ============================================================================
// Re-exports at crate root for backwards compatibility and convenience
// ============================================================================

// Re-export CancellationToken for users
pub use tokio_util::sync::CancellationToken;

// Re-export commonly used types from core at crate root
pub use core::{
    BuiltinError, CommandArg, CreateOptions, Error, ErrorKind, ExecutionContext,
    ExecutionControlFlow, ExecutionExitCode, ExecutionParameters, ExecutionResult,
    ExecutionSpawnResult, ProcessGroupPolicy, Shell, ShellBuilder, ShellBuilderState, ShellFd,
    ShellValue, ShellVariable,
};

// Re-export openfiles module for file descriptor manipulation
pub use core::openfiles;

// Re-export parser types
pub use parser::{
    ParseError, Parser, ParserBuilder, ParserOptions, SourceInfo, Token, TokenLocation,
    TokenizerError, TokenizerOptions,
};

// Re-export builtins types
pub use builtins::{BuiltinSet, ShellBuilderExt, default_builtins};

// Re-export Command trait and builtin function for custom builtins
pub use core::builtins::{builtin, Command, ContentType, DeclarationCommand, Registration};

// Type alias for convenience (matches ExitCode usage in kodegen-tools-terminal)
pub type ExitCode = ExecutionExitCode;
