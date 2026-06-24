//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:245:follow`
//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:245-255, hand-ported)

use crate::functions::follow_type_pack_alt_h::follow_pack_full;
use crate::type_aliases::type_pack_id::TypePackId;

fn identity_mapper(_context: *const core::ffi::c_void, t: TypePackId) -> TypePackId {
    t
}

pub unsafe fn follow(tp: TypePackId) -> TypePackId {
    follow_pack_full(tp, core::ptr::null(), identity_mapper)
}

#[allow(unused_imports)]
pub use follow as follow_type_pack_id;
