use crate::records::scoped_f_value::ScopedFValue;
use core::marker::PhantomData;
use luaur_common::records::f_value::FValueOverridable;

impl<T: FValueOverridable> ScopedFValue<T> {
    pub fn scoped_f_value_scoped_f_value_mut(rhs: &mut ScopedFValue<T>) -> Self {
        let value = rhs.value;
        let old_value = rhs.old_value;

        rhs.value = core::ptr::null_mut();

        Self {
            value,
            old_value,
            _marker: PhantomData,
        }
    }
}
