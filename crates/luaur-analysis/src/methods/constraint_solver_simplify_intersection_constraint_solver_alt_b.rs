use crate::functions::simplify_intersection_simplify::simplify_intersection;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::scope::Scope;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl ConstraintSolver {
    pub fn simplify_intersection_not_null_scope_location_type_ids(
        &mut self,
        _scope: *mut Scope,
        _location: Location,
        parts: TypeIds,
    ) -> TypeId {
        let left = parts.front();
        let mut parts = parts;
        parts.erase_type_id(left);
        let right = parts.front();

        simplify_intersection(self.builtin_types, self.arena, left, right).result
    }
}
