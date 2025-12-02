//! Example of instantiating a shell and calling a shell function in it.

use anyhow::Result;
use kodegen_bash_shell::{Shell, openfiles};

async fn instantiate_shell() -> Result<Shell> {
    let shell = Shell::builder().build().await?;
    Ok(shell)
}

async fn define_func(shell: &mut Shell) -> Result<()> {
    let script = r#"hello() { echo "Hello, world: $@"; return 42; }
"#;

    let result = shell
        .exec(script, &shell.default_exec_params())
        .await?;

    eprintln!("[Function definition result: {}]", result.is_success());

    Ok(())
}

async fn run_func(shell: &mut Shell, suppress_stdout: bool) -> Result<()> {
    let mut params = shell.default_exec_params();

    if suppress_stdout {
        params.set_fd(
            openfiles::OpenFiles::STDOUT_FD,
            openfiles::null()?,
        );
    }

    let result = shell
        .invoke_function("hello", std::iter::once("arg"), &params)
        .await?;

    eprintln!("[Function invocation result: {result}]");

    Ok(())
}

async fn run(suppress_stdout: bool) -> Result<()> {
    let mut shell = instantiate_shell().await?;

    define_func(&mut shell).await?;

    for (name, _) in shell.funcs().iter() {
        eprintln!("[Found function: {name}]");
    }

    run_func(&mut shell, suppress_stdout).await?;

    Ok(())
}

fn main() -> Result<()> {
    const SUPPRESS_STDOUT: bool = true;

    // Construct a runtime for us to run async code on.
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(run(SUPPRESS_STDOUT))?;

    Ok(())
}
