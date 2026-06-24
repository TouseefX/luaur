use crate::functions::follow_type::follow_type_id;
use crate::records::blocked_type_finder::BlockedTypeFinder;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::has_indexer_constraint::HasIndexerConstraint;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn try_dispatch_has_indexer_constraint_not_null_constraint(
        &mut self,
        c: &HasIndexerConstraint,
        constraint: *const Constraint,
    ) -> bool {
        let subject_type = unsafe { follow_type_id(c.subject_type) };
        let index_type = unsafe { follow_type_id(c.index_type) };

        if self.is_blocked_type_id(subject_type) {
            return self.block_type_id_not_null_constraint(subject_type, constraint);
        }
        if self.is_blocked_type_id(index_type) {
            return self.block_type_id_not_null_constraint(index_type, constraint);
        }

        let mut btf = BlockedTypeFinder::blocked_type_finder_blocked_type_finder();
        btf.visit_type_id(subject_type);

        if let Some(blocked) = btf.blocked {
            return self.block_type_id_not_null_constraint(blocked, constraint);
        }

        let mut recursion_depth = 0;
        let mut seen = luaur_common::records::dense_hash_set::DenseHashSet::new(core::ptr::null());

        let result = self.constraint_solver_try_dispatch_has_indexer(
            &mut recursion_depth,
            constraint,
            subject_type,
            index_type,
            c.result_type,
            &mut seen,
        );

        if FFlag::LuauConstraintGraph.get() && result {
            self.unblock_type_id_location(
                subject_type,
                luaur_ast::records::location::Location::default(),
            );
        }

        result
    }
}
