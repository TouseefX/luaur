use crate::records::scoped_exit::ScopedExit;

impl ScopedExit {
    pub fn operator_assign_mut(&mut self, mut other: ScopedExit) -> &mut Self {
        let mut temp = ScopedExit {
            func: core::mem::take(&mut other.func),
        };
        core::mem::swap(&mut self.func, &mut temp.func);
        self
    }
}
