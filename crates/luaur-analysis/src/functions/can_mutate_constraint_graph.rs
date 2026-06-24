use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::type_aliases::type_id::TypeId;
use core::ptr::NonNull;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ anonymous-namespace `bool canMutate(TypeId ty, NotNull<const Constraint> constraint)`.
pub fn can_mutate_type_id_not_null_constraint_mut(
    ty: TypeId,
    constraint: NonNull<Constraint>,
) -> bool {
    let blocked = unsafe { get_type_id::<BlockedType>(ty) };
    if !blocked.is_null() {
        let owner = unsafe { (*blocked).getOwner() };
        LUAU_ASSERT!(!owner.is_null());
        return owner == constraint.as_ptr() as *const Constraint;
    }

    true
}
