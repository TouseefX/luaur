//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeUtils.h:248:follow`
//! Source: `Analysis/include/Luau/TypeUtils.h:247-254` (hand-ported)

use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

/// Dispatch of the inner `follow(*ty)` per id type `Ty`: C++ resolves the
/// overload by argument type; Rust needs the trait.
pub trait FollowId: Copy {
    /// # Safety
    /// `self` must be a valid id pointer (the C++ overloads dereference it).
    unsafe fn follow_id(self) -> Self;
}

impl FollowId for TypeId {
    unsafe fn follow_id(self) -> TypeId {
        follow_type_id(self)
    }
}

impl FollowId for TypePackId {
    unsafe fn follow_id(self) -> TypePackId {
        follow_type_pack_id(self)
    }
}

/// C++ `template<typename Ty> std::optional<Ty> follow(std::optional<Ty> ty)`.
pub unsafe fn follow_optional_ty<Ty: FollowId>(ty: Option<Ty>) -> Option<Ty> {
    if let Some(ty) = ty {
        Some(ty.follow_id())
    } else {
        None
    }
}
