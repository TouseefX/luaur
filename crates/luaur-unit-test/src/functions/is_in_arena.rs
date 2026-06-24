use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::type_id::TypeId;

pub fn is_in_arena(t: TypeId, arena: &TypeArena) -> bool {
    arena.types.contains(t)
}
