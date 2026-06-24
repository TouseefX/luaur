//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Type.cpp:61:follow`
//! Source: `Analysis/src/Type.cpp` (Type.cpp:61-72, hand-ported)

use crate::enums::follow_option::FollowOption;
use crate::functions::follow_type_alt_e::follow_full;
use crate::type_aliases::type_id::TypeId;

fn identity_mapper(_context: *const core::ffi::c_void, t: TypeId) -> TypeId {
    t
}

#[allow(non_snake_case)]
pub unsafe fn follow_with_option(t: TypeId, followOption: FollowOption) -> TypeId {
    follow_full(t, followOption, core::ptr::null(), identity_mapper)
}

#[allow(unused_imports)]
pub use follow_with_option as follow_type_id_follow_option;
