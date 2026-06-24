use crate::records::scoped_exit::ScopedExit;

impl ScopedExit {
    pub fn scoped_exit_scoped_exit_mut(&mut self, mut other: ScopedExit) {
        let mut temp = ScopedExit::scoped_exit();
        ScopedExit::scoped_exit_scoped_exit(&other);
        core::mem::swap(&mut self.func, &mut other.func);
    }
}
