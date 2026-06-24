use crate::records::scoped_f_value::ScopedFValue;
use luaur_common::records::f_value::FValueOverridable;

impl<T: FValueOverridable> ScopedFValue<T> {
    pub fn operator_assign_mut(&mut self, mut rhs: ScopedFValue<T>) -> &mut Self {
        self.value = rhs.value;
        self.old_value = rhs.old_value;

        rhs.value = core::ptr::null_mut();

        self
    }
}
