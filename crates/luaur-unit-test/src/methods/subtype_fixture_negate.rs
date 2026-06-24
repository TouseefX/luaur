use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::records::negation_type::NegationType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn negate(&mut self, ty: TypeId) -> TypeId {
        self.arena.add_type(NegationType::new(ty))
    }
}
