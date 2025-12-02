# Acknowledgements

## Original Project: brush-shell

**kodegen-bash-shell** is a permanent fork of [brush-shell](https://github.com/reubeno/brush), a POSIX- and bash-compatible shell implemented in Rust.

### Original Author

**Reuben Olinsky** ([@reubeno](https://github.com/reubeno))

We extend our sincere gratitude to Reuben Olinsky for creating brush-shell and releasing it under the MIT License, making this fork possible.

### Original Project

- **Repository**: https://github.com/reubeno/brush
- **License**: MIT License
- **Version Forked**: v0.4.0 (brush-core), v0.3.0 (brush-parser), v0.1.0 (brush-builtins)
- **Fork Date**: December 2025

### Why We Forked

brush-shell is an excellent POSIX/bash-compatible shell. However, for our use case in kodegen-tools-terminal (an MCP tool for AI code generation agents), we needed the ability to programmatically interrupt running commands via a cancellation token.

The upstream brush-shell executes commands synchronously via `run_string()`, and while it handles SIGINT gracefully when running as an interactive terminal, there is no mechanism to cancel execution programmatically from an embedding context.

This fork adds:
- `CancellationToken` support in `ExecutionParameters`
- Cooperative cancellation checking in `ChildProcess::wait()`
- The ability to interrupt long-running commands from the embedding application

### Original Credits (from brush-shell)

brush-shell relies on excellent OSS crates:

- [`reedline`](https://github.com/nushell/reedline) - readline-like input
- [`clap`](https://github.com/clap-rs/clap) - command-line parsing
- [`fancy-regex`](https://github.com/fancy-regex/fancy-regex) - regex support
- [`tokio`](https://github.com/tokio-rs/tokio) - async runtime
- [`nix`](https://github.com/nix-rust/nix) - Unix/POSIX system APIs

### License

This fork maintains the original MIT License. See [LICENSE](LICENSE) for full terms.

Both the original work by Reuben Olinsky and modifications by KODEGEN.AI are covered under the MIT License.
