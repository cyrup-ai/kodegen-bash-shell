# kodegen-bash-shell

A permanent fork of [brush-shell](https://github.com/reubeno/brush) with programmatic command cancellation support.

## Overview

**kodegen-bash-shell** provides an embeddable POSIX/bash-compatible shell interpreter for Rust applications, with the critical addition of **cancellation token support** for interrupting running commands programmatically.

This fork exists because the upstream brush-shell, while excellent, does not provide a mechanism to cancel `run_string()` execution from an embedding context. For AI code generation agents that need to interrupt long-running commands, this capability is essential.

## Fork Status

This is a **permanent fork**, not a temporary divergence. We do not expect these changes to be upstreamed, as they represent a fundamental architectural addition that may not align with the upstream project's goals.

| Component | Upstream Version | Fork Package |
|-----------|------------------|--------------|
| brush-core | 0.4.0 | kodegen-bash-shell |
| brush-parser | 0.3.0 | kodegen-bash-parser |
| brush-builtins | 0.1.0 | kodegen-bash-builtins |

## Key Addition: CancellationToken

```rust
use kodegen_bash_shell::{Shell, ExecutionParameters, CancellationToken};

// Create a cancellation token
let token = CancellationToken::new();
let token_clone = token.clone();

// Pass token to execution parameters
let mut params = shell.default_exec_params();
params.set_cancellation_token(token_clone);

// Run command (can be cancelled)
let handle = tokio::spawn(async move {
    shell.run_string("sleep 60", &params).await
});

// Cancel from another task
token.cancel();

// Command returns with Interrupted result
let result = handle.await?;
assert!(result.is_interrupted());
```

## Credits

This project is built on the excellent work of **Reuben Olinsky** ([@reubeno](https://github.com/reubeno)) and the brush-shell project.

See [ACKNOWLEDGEMENTS.md](ACKNOWLEDGEMENTS.md) for full attribution.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

The original brush-shell code by Reuben Olinsky is licensed under MIT.
Fork modifications by KODEGEN.AI are dual-licensed under Apache-2.0 OR MIT.

See [LICENSE](LICENSE) for full details.
