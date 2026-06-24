use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::bound::Bound;
use crate::records::r#type::Type;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::macros::luau_noinline::LUAU_NOINLINE;

LUAU_NOINLINE! {
    pub fn unifiable_bound_type_id_emplace_type_bound_type(
        ty: &mut Type,
        ty_arg: &mut TypeId,
    ) -> *mut Bound<TypeId> {
        unsafe {
            LUAU_ASSERT!((ty as *const Type as TypeId) != follow_type_id(*ty_arg));
            // ty->ty.emplace<BoundType>(tyArg)
            ty.ty = TypeVariant::Bound(*ty_arg);
            get_mutable_type_id::<BoundType>(ty as *const Type as TypeId)
        }
    }
}
