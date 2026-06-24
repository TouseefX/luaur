use crate::functions::snapshot_scope::snapshot_scope;
use crate::functions::snapshot_type_strings::snapshot_type_strings;
use crate::functions::to_string_to_string_alt_q::to_string_constraint_to_string_options;
use crate::records::constraint::Constraint;
use crate::records::constraint_snapshot::ConstraintSnapshot;
use crate::records::constraint_step_snapshot::ConstraintStepSnapshot;
use crate::records::dcr_logger::DcrLogger;
use crate::records::scope::Scope;
use crate::records::scope_snapshot::ScopeSnapshot;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_void;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl DcrLogger {
    /// `ConstraintStepSnapshot DcrLogger::prepareStepSnapshot(...)`
    /// (`Analysis/src/DcrLogger.cpp:424-453`).
    pub fn prepare_step_snapshot(
        &mut self,
        root_scope: &Scope,
        current: *const Constraint,
        force: bool,
        unsolved_constraints: &Vec<*const Constraint>,
    ) -> ConstraintStepSnapshot {
        let scope_snapshot: ScopeSnapshot = snapshot_scope(root_scope, &mut self.opts);
        let mut constraints: DenseHashMap<*const Constraint, ConstraintSnapshot> =
            DenseHashMap::new(core::ptr::null());

        for &c in unsolved_constraints.iter() {
            let stringification =
                to_string_constraint_to_string_options(unsafe { &*c }, &mut self.opts);
            let location = unsafe { (*c).location };
            let blocks = self.snapshot_blocks(c);
            *constraints.get_or_insert(c) = ConstraintSnapshot {
                stringification,
                location,
                blocks,
            };
        }

        let mut type_strings: DenseHashMap<*const c_void, String> =
            DenseHashMap::new(core::ptr::null());
        let Self {
            generation_log,
            opts,
            ..
        } = self;
        snapshot_type_strings(
            &generation_log.expr_type_locations,
            &generation_log.annotation_type_locations,
            &mut type_strings,
            opts,
        );

        ConstraintStepSnapshot {
            current_constraint: current,
            forced: force,
            unsolved_constraints: constraints,
            root_scope: scope_snapshot,
            type_strings,
        }
    }
}
