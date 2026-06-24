//! Source: `Analysis/include/Luau/Constraint.h:374-382` (hand-ported)
use crate::records::constraint::Constraint;
use crate::type_aliases::constraint_v::ConstraintVMember;

/// C++ `template<typename T> const T* get(const Constraint& c)`.
pub fn get_constraint<T: ConstraintVMember>(c: &Constraint) -> *const T {
    match T::get_if(&c.c) {
        Some(r) => r as *const T,
        None => core::ptr::null(),
    }
}
