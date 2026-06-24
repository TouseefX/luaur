//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/Constraint.h:372:get_mutable`
//! Source: `Analysis/include/Luau/Constraint.h:372-377` (hand-ported)

use crate::records::constraint::Constraint;
use crate::type_aliases::constraint_v::ConstraintVMember;

/// C++ `template<typename T> T* getMutable(Constraint& c)`.
pub fn get_mutable_constraint<T: ConstraintVMember>(c: &mut Constraint) -> *mut T {
    match T::get_if_mut(&mut c.c) {
        Some(r) => r as *mut T,
        None => core::ptr::null_mut(),
    }
}

#[allow(unused_imports)]
pub use get_mutable_constraint as get_mutable;
