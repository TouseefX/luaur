use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::try_strip_union_from_nil::try_strip_union_from_nil;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_arena::TypeArena;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

pub fn strip_nil(builtin_types: *mut BuiltinTypes, arena: &mut TypeArena, ty: TypeId) -> TypeId {
    let builtin_types = unsafe { builtin_types.as_ref().expect("builtin_types is null") };
    let ty = unsafe { follow_type_id(ty) };

    if unsafe { get_type_id::<UnionType>(ty) }.is_null() {
        return unsafe { follow_type_id(ty) };
    }

    let cleaned = try_strip_union_from_nil(arena, ty);

    // If there is no union option without 'nil'
    if cleaned.is_none() {
        return builtin_types.nilType;
    }

    unsafe { follow_type_id(cleaned.unwrap()) }
}
