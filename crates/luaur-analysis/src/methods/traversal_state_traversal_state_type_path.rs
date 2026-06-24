use crate::functions::follow_type::follow_type_id;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::traversal_state::TraversalState;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;

impl TraversalState {
    pub fn traversal_state_type_id_not_null_builtin_types_type_arena(
        root: TypeId,
        builtin_types: &BuiltinTypes,
        arena: &mut TypeArena,
    ) -> Self {
        TraversalState {
            current: crate::type_aliases::type_or_pack::TypeOrPack::V0(unsafe {
                follow_type_id(root)
            }),
            builtin_types,
            arena,
            steps: 0,
            encountered_error_suppression: false,
        }
    }
}
