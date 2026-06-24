use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_arena::TypeArena;
use crate::records::union_builder::UnionBuilder;
use crate::type_aliases::type_id::TypeId;

pub fn add_union(
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    list: &[TypeId],
) -> TypeId {
    let mut ub = UnionBuilder::union_builder(arena, builtin_types);
    ub.reserve(list.len());
    for &option in list.iter() {
        ub.add(option);
    }
    ub.build()
}
