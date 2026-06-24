use crate::records::simplify_fixture::SimplifyFixture;
use luaur_analysis::records::negation_type::NegationType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SimplifyFixture {
    pub fn mk_negation(&mut self, ty: TypeId) -> TypeId {
        self.arena.add_type(NegationType::new(ty))
    }
}
