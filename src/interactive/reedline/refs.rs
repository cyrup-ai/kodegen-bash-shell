use std::{
    borrow::{Borrow, BorrowMut},
    sync::Arc,
};

use tokio::sync::{Mutex, MutexGuard};

pub(crate) type ShellRef = Arc<Mutex<crate::core::Shell>>;

pub(crate) struct ReedlineShellReader<'a> {
    pub shell: MutexGuard<'a, crate::core::Shell>,
}

impl AsRef<crate::core::Shell> for ReedlineShellReader<'_> {
    fn as_ref(&self) -> &crate::core::Shell {
        self.shell.borrow()
    }
}

pub(crate) struct ReedlineShellWriter<'a> {
    pub shell: MutexGuard<'a, crate::core::Shell>,
}

impl AsMut<crate::core::Shell> for ReedlineShellWriter<'_> {
    fn as_mut(&mut self) -> &mut crate::core::Shell {
        self.shell.borrow_mut()
    }
}
