use crate::functions::simplify_intersection_simplify::simplify_intersection;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::scope::Scope;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl ConstraintSolver {
    pub fn simplify_intersection_not_null_scope_location_type_id_type_id(
        &mut self,
        _scope: *mut Scope,
        _location: Location,
        left: TypeId,
        right: TypeId,
    ) -> TypeId {
        simplify_intersection(self.builtin_types, self.arena, left, right).result
    }
}
