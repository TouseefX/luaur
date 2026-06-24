//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypeOrPack.cpp:9:ptr`
//! Source: `Analysis/src/TypeOrPack.cpp:9-17` (hand-ported)

use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::{TypeOrPack, TypeOrPackMember};
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;

/// C++ `const void* ptr(TypeOrPack tyOrTp)`.
pub fn ptr(ty_or_tp: TypeOrPack) -> *const core::ffi::c_void {
    if let Some(ty) = TypeId::get_if(&ty_or_tp) {
        *ty as *const core::ffi::c_void
    } else if let Some(tp) = TypePackId::get_if(&ty_or_tp) {
        *tp as *const core::ffi::c_void
    } else {
        LUAU_UNREACHABLE!()
    }
}
