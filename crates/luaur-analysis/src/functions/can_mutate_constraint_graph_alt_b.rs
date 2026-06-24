use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::constraint::Constraint;
use crate::type_aliases::type_pack_id::TypePackId;
use core::ptr::NonNull;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ anonymous-namespace `static bool canMutate(TypePackId tp, NotNull<const Constraint> constraint)`.
pub fn can_mutate_type_pack_id_not_null_constraint_mut(
    tp: TypePackId,
    constraint: NonNull<Constraint>,
) -> bool {
    let blocked = unsafe { get_type_pack_id::<BlockedTypePack>(tp) };
    if !blocked.is_null() {
        let owner = unsafe { (*blocked).owner };
        LUAU_ASSERT!(!owner.is_null());
        return owner as *const Constraint == constraint.as_ptr() as *const Constraint;
    }

    true
}
