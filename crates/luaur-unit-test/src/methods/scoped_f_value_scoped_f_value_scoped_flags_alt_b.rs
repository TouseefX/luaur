use crate::records::scoped_f_value::ScopedFValue;
use luaur_common::records::f_value::FValueOverridable;

impl<T: FValueOverridable> ScopedFValue<T> {
    pub fn scoped_f_value_scoped_f_value(_rhs: &ScopedFValue<T>) -> Self {
        // C++: ScopedFValue(const ScopedFValue&) = delete;
        // This overload is deleted in C++; preserve that behavior at runtime.
        panic!("ScopedFValue copy constructor is deleted in C++");
    }
}
