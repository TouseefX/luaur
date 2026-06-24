use crate::records::scoped_f_value::ScopedFValue;
use luaur_common::records::f_value::FValueOverridable;

impl<T: FValueOverridable> Drop for ScopedFValue<T> {
    fn drop(&mut self) {
        // Remove the thread-local override this guard installed (a moved-from
        // guard has a null `value` and pops nothing — exactly one pop per push).
        if !self.value.is_null() {
            unsafe {
                (*self.value).pop_test_override();
            }
        }
    }
}
