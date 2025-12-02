//! Example of streaming command output using `stream()`.
//!
//! This example demonstrates:
//! - Running a command with real-time streaming output
//! - Processing stdout/stderr chunks as they arrive
//! - Using cancellation tokens to stop execution
//!
//! Run this example with:
//! ```bash
//! cargo run --example streaming
//! ```

use anyhow::Result;
use futures::StreamExt;
use std::io::Write;

use kodegen_bash_shell::{OutputStreamType, Shell};

async fn run_example() -> Result<()> {
    // Create a shell instance
    let mut shell = Shell::builder().build().await?;
    let params = shell.default_exec_params();

    println!("Running 'cargo clean && cargo clippy' with streaming output...\n");

    // Get streaming output and stdin sender
    let (mut stream, _stdin_tx) = shell.stream("cargo clean && cargo clippy 2>&1", &params)?;

    // Process output chunks as they arrive
    while let Some(output) = stream.next().await {
        match output.stream {
            OutputStreamType::Stdout => {
                print!("{}", output.as_str_lossy());
                std::io::stdout().flush()?;
            }
            OutputStreamType::Stderr => {
                eprint!("{}", output.as_str_lossy());
                std::io::stderr().flush()?;
            }
        }
    }

    println!("\nStreaming complete!");

    // =========================================================================
    // Part 2: Demonstrate cancellation
    // =========================================================================

    println!("\n--- Cancellation Demo ---");
    println!("Running 'cargo clean && cargo clippy' but cancelling after 2 seconds...\n");

    // Set up cancellation token
    let token = kodegen_bash_shell::CancellationToken::new();
    let mut params = shell.default_exec_params();
    params.set_cancellation_token(token.clone());

    let (mut stream, _stdin_tx) = shell.stream("cargo clean && cargo clippy 2>&1", &params)?;

    // Spawn a task to cancel after 2 seconds
    let cancel_token = token.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        println!("\n[Cancelling execution...]");
        cancel_token.cancel();
    });

    // Process output until cancelled
    while let Some(output) = stream.next().await {
        match output.stream {
            OutputStreamType::Stdout => {
                print!("{}", output.as_str_lossy());
                std::io::stdout().flush()?;
            }
            OutputStreamType::Stderr => {
                eprint!("{}", output.as_str_lossy());
                std::io::stderr().flush()?;
            }
        }
    }

    println!("\nStream ended (cancelled)!");

    Ok(())
}

fn main() -> Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(run_example())?;

    Ok(())
}
