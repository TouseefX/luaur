use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn can_mutate(ty: TypeId, constraint: *const Constraint) -> bool {
    let ty = unsafe { follow_type_id(ty) };
    if let Some(blocked) = unsafe { get_type_id::<BlockedType>(ty).as_ref() } {
        let owner = blocked.get_owner();
        LUAU_ASSERT!(!owner.is_null());
        return owner == constraint;
    }

    true
}
