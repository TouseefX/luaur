//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:461:emplaceTypePack`
//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:461-466, hand-ported)

use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::records::bound::Bound;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::bound_type_pack::BoundTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::macros::luau_noinline::LUAU_NOINLINE;

LUAU_NOINLINE! {
    pub fn emplace_type_pack(ty: *mut TypePackVar, ty_arg: &mut TypePackId) -> *mut Bound<TypePackId> {
        unsafe {
            LUAU_ASSERT!((ty as *const TypePackVar as TypePackId) != follow_type_pack_id(*ty_arg));
            // ty->ty.emplace<BoundTypePack>(tyArg)
            (*ty).ty = TypePackVariant::Bound(*ty_arg);
            get_mutable_type_pack_id::<BoundTypePack>(ty as *const TypePackVar as TypePackId)
        }
    }
}
