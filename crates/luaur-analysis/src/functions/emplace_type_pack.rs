use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use luaur_common::macros::luau_noinline::LUAU_NOINLINE;

LUAU_NOINLINE! {
    pub fn emplace_type_pack(ty: *mut TypePackVar, variant: TypePackVariant) -> *mut TypePackVariant {
        unsafe {
            let ty_ref = &mut *ty;
            ty_ref.operator_assign_type_pack_variant(variant);
            &mut (*ty).ty
        }
    }
}
