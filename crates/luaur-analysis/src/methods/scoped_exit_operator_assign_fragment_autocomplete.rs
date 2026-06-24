use crate::records::scoped_exit::ScopedExit;

impl ScopedExit {
    /// @delete
    #[allow(dead_code)]
    pub fn operator_assign(&mut self, _other: &ScopedExit) -> &mut ScopedExit {
        unimplemented!("ScopedExit copy assignment is deleted in C++")
    }
}
