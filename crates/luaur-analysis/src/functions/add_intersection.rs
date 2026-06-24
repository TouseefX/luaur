use crate::records::builtin_types::BuiltinTypes;
use crate::records::intersection_builder::IntersectionBuilder;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;

pub fn add_intersection(
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    list: &[TypeId],
) -> TypeId {
    let mut ib = IntersectionBuilder::intersection_builder(arena, builtin_types);
    ib.reserve(list.len());
    for &part in list {
        ib.add(part);
    }
    ib.build()
}
