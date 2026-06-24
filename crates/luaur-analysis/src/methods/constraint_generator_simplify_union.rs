use crate::functions::simplify_union::simplify_union;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn simplify_union(
        &mut self,
        _scope: ScopePtr,
        _location: Location,
        left: TypeId,
        right: TypeId,
    ) -> TypeId {
        simplify_union(self.builtin_types, self.arena, left, right).result
    }
}
