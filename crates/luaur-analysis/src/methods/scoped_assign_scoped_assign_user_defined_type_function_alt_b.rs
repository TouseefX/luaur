use crate::records::scoped_assign::ScopedAssign;

impl<T> Drop for ScopedAssign<T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::swap(self.target, &mut self.old_value);
        }
    }
}

#[allow(non_snake_case)]
pub fn scoped_assign_scoped_assign() {}
