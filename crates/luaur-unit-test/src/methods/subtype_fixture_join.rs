use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::union_type::UnionType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn join(&mut self, a: TypeId, b: TypeId) -> TypeId {
        self.arena.add_type(UnionType {
            options: vec![a, b],
        })
    }
}
