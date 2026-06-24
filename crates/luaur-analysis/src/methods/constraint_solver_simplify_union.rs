use crate::functions::simplify_union::simplify_union;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl ConstraintSolver {
    pub fn simplify_union(
        &mut self,
        scope: *mut Scope,
        location: Location,
        left: TypeId,
        right: TypeId,
    ) -> TypeId {
        let builtin_types = self.builtin_types;
        let arena = self.arena;
        let _ = scope;
        let _ = location;
        simplify_union(builtin_types, arena, left, right).result
    }
}
