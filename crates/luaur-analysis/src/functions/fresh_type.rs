use crate::enums::polarity::Polarity;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::free_type::FreeType;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;

pub fn fresh_type(
    arena: &mut TypeArena,
    builtin_types: &BuiltinTypes,
    scope: *mut Scope,
    polarity: Polarity,
) -> TypeId {
    let free_type = FreeType::free_type_scope_type_id_type_id_polarity(
        scope,
        builtin_types.neverType,
        builtin_types.unknownType,
        polarity,
    );
    arena.add_type(free_type)
}
