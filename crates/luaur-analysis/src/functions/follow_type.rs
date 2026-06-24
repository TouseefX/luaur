//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Type.cpp:56:follow`
//! Source: `Analysis/src/Type.cpp` (Type.cpp:56-59, hand-ported)

use crate::enums::follow_option::FollowOption;
use crate::functions::follow_type_alt_e::follow_full;
use crate::type_aliases::type_id::TypeId;

fn identity_mapper(_context: *const core::ffi::c_void, t: TypeId) -> TypeId {
    t
}

pub unsafe fn follow(t: TypeId) -> TypeId {
    follow_full(t, FollowOption::Normal, core::ptr::null(), identity_mapper)
}

#[allow(unused_imports)]
pub use follow as follow_type_id;
