use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::maybe_singleton::maybe_singleton;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::free_type::FreeType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::primitive_type_constraint::PrimitiveTypeConstraint;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn try_dispatch_primitive_type_constraint_not_null_constraint(
        &mut self,
        c: &PrimitiveTypeConstraint,
        constraint: *const Constraint,
    ) -> bool {
        let expected_type = if c.expected_type.is_some() {
            Some(unsafe { follow_type_id(c.expected_type.unwrap()) })
        } else {
            None
        };

        if let Some(et) = expected_type {
            if self.is_blocked_type_id(et)
                || !unsafe { get_type_id::<PendingExpansionType>(et) }.is_null()
            {
                return self.block_type_id_not_null_constraint(et, constraint);
            }
        }

        let free_type = unsafe { get_type_id::<FreeType>(follow_type_id(c.free_type)) };

        if free_type.is_null() {
            return true;
        }

        if FFlag::LuauConstraintGraph.get() {
            if unsafe {
                (*self.cgraph).has_strictly_more_than_one_dependency(
                    crate::type_aliases::constraint_vertex::ConstraintVertex::V0(c.free_type),
                )
            } {
                self.block_type_id_not_null_constraint(c.free_type, constraint);
                return false;
            }
        } else {
            if let Some(it) = self.deprecated_type_to_constraint_set.get(&c.free_type) {
                if it.len() > 1 {
                    self.block_type_id_not_null_constraint(c.free_type, constraint);
                    return false;
                }
            }
        }

        let mut bind_to = c.primitive_type;

        let free_type_ref = unsafe { &*free_type };
        if free_type_ref.upper_bound != c.primitive_type
            && maybe_singleton(free_type_ref.upper_bound)
        {
            bind_to = free_type_ref.lower_bound;
        } else if let Some(et) = expected_type {
            if maybe_singleton(et) {
                bind_to = free_type_ref.lower_bound;
            }
        }

        let ty = unsafe { follow_type_id(c.free_type) };
        if !FFlag::LuauConstraintGraph.get() {
            self.deprecate_d_shift_references(ty, bind_to);
        }
        self.bind_not_null_constraint_type_id_type_id(constraint, ty, bind_to);

        true
    }
}
