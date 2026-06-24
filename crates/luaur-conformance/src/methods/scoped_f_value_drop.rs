use crate::records::scoped_f_value::ScopedFValue;
use luaur_common::records::f_value::FValueOverridable;

impl<T: FValueOverridable> Drop for ScopedFValue<T> {
    fn drop(&mut self) {
        if !self.value.is_null() {
            unsafe {
                (*self.value).pop_test_override();
            }
        }
    }
}
