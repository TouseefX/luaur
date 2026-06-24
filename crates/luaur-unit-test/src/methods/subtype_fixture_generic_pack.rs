use crate::records::subtype_fixture::SubtypeFixture;
use alloc::string::ToString;
use luaur_analysis::records::generic_type_pack::GenericTypePack;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;

impl SubtypeFixture {
    pub fn generic_pack(&mut self, name: &str) -> TypePackId {
        self.arena
            .add_type_pack_t(GenericTypePack::new_name(name.to_string()))
    }
}
