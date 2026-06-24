use crate::records::simplify_fixture::SimplifyFixture;
use luaur_analysis::functions::simplify_union::simplify_union;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SimplifyFixture {
    pub fn union_(&mut self, a: TypeId, b: TypeId) -> TypeId {
        simplify_union(self.base.builtin_types, &mut self.arena, a, b).result
    }
}
