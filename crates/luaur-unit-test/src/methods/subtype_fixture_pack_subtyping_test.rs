use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;

impl SubtypeFixture {
    pub fn pack_initializer_list_type_id(&mut self, tys: Vec<TypeId>) -> TypePackId {
        self.arena.add_type_pack_initializer_list_type_id(&tys)
    }
}
