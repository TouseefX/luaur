//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Type.cpp:74:follow`
//! Source: `Analysis/src/Type.cpp` (Type.cpp:74-77, hand-ported)

use crate::enums::follow_option::FollowOption;
use crate::functions::follow_type_alt_e::follow_full;
use crate::type_aliases::type_id::TypeId;

pub unsafe fn follow_with_mapper(
    t: TypeId,
    context: *const core::ffi::c_void,
    mapper: fn(*const core::ffi::c_void, TypeId) -> TypeId,
) -> TypeId {
    follow_full(t, FollowOption::Normal, context, mapper)
}

#[allow(unused_imports)]
pub use follow_with_mapper as follow_type_id_void_type_id_item_mapper_const_void;
