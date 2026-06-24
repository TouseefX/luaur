use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn opt(&mut self, ty: TypeId) -> TypeId {
        self.join(ty, self.builtin_types.nilType)
    }
}
