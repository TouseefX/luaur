use crate::functions::get_type_pack::get;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::constraint::Constraint;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn can_mutate(tp: TypePackId, constraint: *const Constraint) -> bool {
    if let Some(blocked) = unsafe { get::<BlockedTypePack>(tp).as_ref() } {
        let owner = blocked.owner;
        LUAU_ASSERT!(!owner.is_null());
        return owner == constraint as *mut Constraint;
    }

    true
}
