pub(crate) trait ShellFactory {
    type ShellType: crate::interactive::InteractiveShell + Send;

    async fn create(
        &self,
        options: crate::interactive::Options,
    ) -> Result<Self::ShellType, crate::interactive::ShellError>;
}

#[allow(dead_code, reason = "unused on some platforms")]
pub(crate) struct StubShell;

#[expect(clippy::panic)]
impl crate::interactive::InteractiveShell for StubShell {
    #[expect(unreachable_code)]
    fn shell(&self) -> impl AsRef<crate::core::Shell> + Send {
        panic!("No interactive shell implementation available");
        self
    }

    #[expect(unreachable_code)]
    fn shell_mut(&mut self) -> impl AsMut<crate::core::Shell> + Send {
        panic!("No interactive shell implementation available");
        self
    }

    fn read_line(
        &mut self,
        _prompt: crate::interactive::InteractivePrompt,
    ) -> Result<crate::interactive::ReadResult, crate::interactive::ShellError> {
        Err(crate::interactive::ShellError::InputBackendNotSupported)
    }
}

#[expect(clippy::panic)]
impl AsRef<crate::core::Shell> for StubShell {
    fn as_ref(&self) -> &crate::core::Shell {
        panic!("No interactive shell implementation available")
    }
}

#[expect(clippy::panic)]
impl AsMut<crate::core::Shell> for StubShell {
    fn as_mut(&mut self) -> &mut crate::core::Shell {
        panic!("No interactive shell implementation available")
    }
}

pub(crate) struct ReedlineShellFactory;

#[allow(unused_variables, reason = "options are not used on all platforms")]
impl ShellFactory for ReedlineShellFactory {
    #[cfg(all(feature = "reedline", any(unix, windows)))]
    type ShellType = crate::interactive::ReedlineShell;
    #[cfg(any(not(feature = "reedline"), not(any(unix, windows))))]
    type ShellType = StubShell;

    async fn create(
        &self,
        options: crate::interactive::Options,
    ) -> Result<Self::ShellType, crate::interactive::ShellError> {
        #[cfg(all(feature = "reedline", any(unix, windows)))]
        {
            crate::interactive::ReedlineShell::new(options).await
        }
        #[cfg(any(not(feature = "reedline"), not(any(unix, windows))))]
        {
            Err(crate::interactive::ShellError::InputBackendNotSupported)
        }
    }
}

pub(crate) struct BasicShellFactory;

#[allow(unused_variables, reason = "options are not used on all platforms")]
impl ShellFactory for BasicShellFactory {
    #[cfg(feature = "basic")]
    type ShellType = crate::interactive::BasicShell;
    #[cfg(not(feature = "basic"))]
    type ShellType = StubShell;

    async fn create(
        &self,
        options: crate::interactive::Options,
    ) -> Result<Self::ShellType, crate::interactive::ShellError> {
        #[cfg(feature = "basic")]
        {
            crate::interactive::BasicShell::new(options).await
        }
        #[cfg(not(feature = "basic"))]
        {
            Err(crate::interactive::ShellError::InputBackendNotSupported)
        }
    }
}

pub(crate) struct MinimalShellFactory;

impl ShellFactory for MinimalShellFactory {
    #[cfg(feature = "minimal")]
    type ShellType = crate::interactive::MinimalShell;
    #[cfg(not(feature = "minimal"))]
    type ShellType = StubShell;

    #[allow(unused_variables, reason = "options are not used on all platforms")]
    async fn create(
        &self,
        options: crate::interactive::Options,
    ) -> Result<Self::ShellType, crate::interactive::ShellError> {
        #[cfg(feature = "minimal")]
        {
            crate::interactive::MinimalShell::new(options).await
        }
        #[cfg(not(feature = "minimal"))]
        {
            Err(crate::interactive::ShellError::InputBackendNotSupported)
        }
    }
}
