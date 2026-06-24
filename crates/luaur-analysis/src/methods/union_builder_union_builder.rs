use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_arena::TypeArena;
use crate::records::union_builder::UnionBuilder;

impl UnionBuilder {
    pub fn union_builder(arena: *mut TypeArena, builtin_types: *mut BuiltinTypes) -> Self {
        Self {
            arena,
            builtin_types,
            options: crate::records::type_ids::TypeIds::type_ids(),
            is_top: false,
        }
    }
}
