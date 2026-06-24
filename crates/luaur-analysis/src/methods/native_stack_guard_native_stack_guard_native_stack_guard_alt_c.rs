use crate::records::native_stack_guard::NativeStackGuard;

impl NativeStackGuard {
    pub fn native_stack_guard_mut(&mut self) {
        self.high = 0;
        self.low = 0;
    }
}
