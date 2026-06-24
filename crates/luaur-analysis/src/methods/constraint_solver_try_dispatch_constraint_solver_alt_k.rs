use crate::enums::table_state::TableState;
use crate::functions::can_mutate_constraint_solver::can_mutate;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::has_prop_constraint::HasPropConstraint;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintSolver {
    pub fn try_dispatch_has_prop_constraint_not_null_constraint(
        &mut self,
        c: &HasPropConstraint,
        constraint: *const Constraint,
    ) -> bool {
        let subject_type = unsafe { follow_type_id(c.subject_type) };
        let result_type = unsafe { follow_type_id(c.result_type) };

        LUAU_ASSERT!(!unsafe { get_type_id::<BlockedType>(result_type) }.is_null());
        LUAU_ASSERT!(can_mutate(result_type, constraint));

        if self.is_blocked_type_id(subject_type) {
            return self.block_type_id_not_null_constraint(subject_type, constraint);
        }

        if let Some(subject_table) = get_table_type(subject_type) {
            if subject_table.state == TableState::Unsealed
                && subject_table.remaining_props > 0
                && !subject_table.props.contains_key(&c.prop)
            {
                return self.block_type_id_not_null_constraint(subject_type, constraint);
            }
        }

        let lookup = self
            .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool(
                constraint,
                subject_type,
                &c.prop,
                c.context,
                c.in_conditional,
                c.suppress_simplification,
            );
        if !lookup.blocked_types.is_empty() {
            for blocked in lookup.blocked_types {
                self.block_type_id_not_null_constraint(blocked, constraint);
            }
            return false;
        }

        self.bind_not_null_constraint_type_id_type_id(
            constraint,
            result_type,
            lookup
                .prop_type
                .unwrap_or(unsafe { (*self.builtin_types).anyType }),
        );
        true
    }
}
