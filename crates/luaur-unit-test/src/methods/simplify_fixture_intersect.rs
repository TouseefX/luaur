use crate::records::simplify_fixture::SimplifyFixture;
use luaur_analysis::functions::simplify_intersection_simplify::simplify_intersection;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SimplifyFixture {
    pub fn intersect(&mut self, a: TypeId, b: TypeId) -> TypeId {
        simplify_intersection(self.base.builtin_types, &mut self.arena, a, b).result
    }
}
