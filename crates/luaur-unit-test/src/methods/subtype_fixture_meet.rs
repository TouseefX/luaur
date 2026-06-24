use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::intersection_type::IntersectionType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn meet(&mut self, a: TypeId, b: TypeId) -> TypeId {
        self.arena.add_type(IntersectionType { parts: vec![a, b] })
    }
}
