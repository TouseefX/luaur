use crate::enums::value_context::ValueContext;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::table_prop_lookup_result::TablePropLookupResult;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl ConstraintSolver {
    pub fn lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool(
        &mut self,
        constraint: *const Constraint,
        subject_type: TypeId,
        prop_name: &str,
        context: ValueContext,
        in_conditional: bool,
        suppress_simplification: bool,
    ) -> TablePropLookupResult {
        let mut seen = DenseHashSet::new(core::ptr::null());
        self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint, subject_type, prop_name, context, in_conditional, suppress_simplification, &mut seen
        )
    }
}
