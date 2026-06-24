//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypeOrPack.cpp:19:follow`
//! Source: `Analysis/src/TypeOrPack.cpp:19-27` (hand-ported)

use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::{TypeOrPack, TypeOrPackMember};
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;

/// C++ `TypeOrPack follow(TypeOrPack tyOrTp)`.
pub unsafe fn follow_type_or_pack(ty_or_tp: TypeOrPack) -> TypeOrPack {
    if let Some(ty) = TypeId::get_if(&ty_or_tp) {
        TypeOrPack::V0(follow_type_id(*ty))
    } else if let Some(tp) = TypePackId::get_if(&ty_or_tp) {
        TypeOrPack::V1(follow_type_pack_id(*tp))
    } else {
        LUAU_UNREACHABLE!()
    }
}

#[allow(unused_imports)]
pub use follow_type_or_pack as follow;
