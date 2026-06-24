use crate::records::scoped_exit::ScopedExit;

impl ScopedExit {
    /// @delete
    #[allow(dead_code)]
    pub fn scoped_exit_scoped_exit(_other: &ScopedExit) {
        unimplemented!("ScopedExit copy constructor is deleted in C++");
    }
}
