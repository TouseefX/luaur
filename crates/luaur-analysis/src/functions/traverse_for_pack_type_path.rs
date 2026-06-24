use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_or_pack::get_type_or_pack_mut;
use crate::functions::traverse_type_path::traverse;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::path::Path;
use crate::records::traversal_state::TraversalState;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn traverse_for_pack(
    root: TypeId,
    path: &Path,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
) -> Option<TypePackId> {
    let mut state = TraversalState::traversal_state_type_id_not_null_builtin_types_type_arena(
        unsafe { follow_type_id(root) },
        builtin_types,
        arena,
    );
    if crate::functions::traverse_type_path::traverse(&mut state, path) {
        if state.encountered_error_suppression {
            return Some(builtin_types.errorTypePack);
        }
        let ty = unsafe { get_type_or_pack_mut::<TypePackId>(&state.current) };
        if !ty.is_null() {
            return Some(unsafe { *ty });
        }
    }
    None
}
