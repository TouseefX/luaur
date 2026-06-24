use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::type_instantiation_constraint::TypeInstantiationConstraint;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag::LuauExplicitTypeInstantiationSupport;

impl ConstraintSolver {
    pub fn try_dispatch_type_instantiation_constraint_not_null_constraint(
        &mut self,
        c: &TypeInstantiationConstraint,
        constraint: *const Constraint,
    ) -> bool {
        LUAU_ASSERT!(LuauExplicitTypeInstantiationSupport.get());

        if self.is_blocked_type_id(c.function_type) {
            return self.block_type_id_not_null_constraint(c.function_type, constraint);
        }

        let bound_to = self.instantiate_function_type(
            c.function_type,
            &c.type_arguments,
            &c.type_pack_arguments,
            unsafe { (*constraint).scope },
            unsafe { &(*constraint).location },
        );
        self.bind_not_null_constraint_type_id_type_id(constraint, c.placeholder_type, bound_to);

        true
    }
}
