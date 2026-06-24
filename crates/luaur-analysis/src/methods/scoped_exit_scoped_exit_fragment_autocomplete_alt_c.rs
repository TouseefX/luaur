use crate::records::scoped_exit::ScopedExit;

impl ScopedExit {
    pub fn scoped_exit() -> Self {
        ScopedExit::scoped_exit_function_void(Box::new(|| {}))
    }
}
