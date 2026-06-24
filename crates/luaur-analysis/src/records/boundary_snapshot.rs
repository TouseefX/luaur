use crate::records::constraint::Constraint;
use crate::records::constraint_snapshot::ConstraintSnapshot;
use crate::records::scope_snapshot::ScopeSnapshot;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct BoundarySnapshot {
    pub(crate) unsolved_constraints: DenseHashMap<*const Constraint, ConstraintSnapshot>,
    pub(crate) root_scope: ScopeSnapshot,
    pub(crate) type_strings: DenseHashMap<*const core::ffi::c_void, alloc::string::String>,
}
