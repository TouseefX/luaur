use crate::records::constraint_generator::ConstraintGenerator;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn make_intersect(
        &mut self,
        scope: &ScopePtr,
        location: Location,
        lhs: TypeId,
        rhs: TypeId,
    ) -> TypeId {
        let builtin_types = unsafe { &*self.builtin_types };
        let intersect_func = &builtin_types.typeFunctions.intersect_func;

        self.create_type_function_instance(
            intersect_func,
            alloc::vec![lhs, rhs],
            alloc::vec![],
            scope,
            location,
        )
    }
}
