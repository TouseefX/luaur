use crate::records::type_arena::TypeArena;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

pub fn make_union(arena: &mut TypeArena, types: Vec<TypeId>) -> TypeId {
    arena.add_type(UnionType { options: types })
}
