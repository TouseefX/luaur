use crate::records::intersection_type::IntersectionType;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

pub fn make_intersection(arena: &mut TypeArena, types: Vec<TypeId>) -> TypeId {
    arena.add_type(IntersectionType { parts: types })
}
