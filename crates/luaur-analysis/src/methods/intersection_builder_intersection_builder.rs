use crate::records::builtin_types::BuiltinTypes;
use crate::records::intersection_builder::IntersectionBuilder;
use crate::records::type_arena::TypeArena;
use crate::records::type_ids::TypeIds;

impl IntersectionBuilder {
    pub fn intersection_builder(arena: *mut TypeArena, builtin_types: *mut BuiltinTypes) -> Self {
        Self {
            arena,
            builtin_types,
            parts: TypeIds::type_ids(),
            is_bottom: false,
        }
    }
}
