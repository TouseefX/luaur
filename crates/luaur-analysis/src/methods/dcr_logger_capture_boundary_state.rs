use crate::functions::snapshot_scope::snapshot_scope;
use crate::functions::snapshot_type_strings::snapshot_type_strings;
use crate::functions::to_string_to_string_alt_q::to_string_constraint_to_string_options;
use crate::records::boundary_snapshot::BoundarySnapshot;
use crate::records::constraint::Constraint;
use crate::records::constraint_snapshot::ConstraintSnapshot;
use crate::records::dcr_logger::DcrLogger;
use crate::records::scope::Scope;
use alloc::vec::Vec;

impl DcrLogger {
    pub fn capture_boundary_state(
        &mut self,
        target: &mut BoundarySnapshot,
        root_scope: &Scope,
        unsolved_constraints: &Vec<*const Constraint>,
    ) {
        target.root_scope = snapshot_scope(root_scope, &mut self.opts);
        target.unsolved_constraints.clear();

        for &c in unsolved_constraints.iter() {
            let constraint_str =
                to_string_constraint_to_string_options(unsafe { &*c }, &mut self.opts);
            let location = unsafe { (*c).location };
            let blocks = self.snapshot_blocks(c);
            let snapshot = ConstraintSnapshot {
                stringification: constraint_str,
                location,
                blocks,
            };
            *target.unsolved_constraints.get_or_insert(c) = snapshot;
        }

        let Self {
            generation_log,
            opts,
            ..
        } = self;
        snapshot_type_strings(
            &generation_log.expr_type_locations,
            &generation_log.annotation_type_locations,
            &mut target.type_strings,
            opts,
        );
    }
}
