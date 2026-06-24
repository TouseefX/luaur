use crate::functions::make_union::make_union;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use core::ptr::NonNull;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn make_option(
    builtin_types: NonNull<BuiltinTypes>,
    arena: &mut TypeArena,
    t: TypeId,
) -> TypeId {
    LUAU_ASSERT!(!t.is_null());
    let builtin_types = unsafe { builtin_types.as_ref() };
    make_union(arena, vec![builtin_types.nilType, t])
}
