use crate::records::boundary_snapshot::BoundarySnapshot;
use crate::type_aliases::step_snapshot::StepSnapshot;

#[derive(Debug, Clone)]
pub struct TypeSolveLog {
    pub(crate) initial_state: BoundarySnapshot,
    pub(crate) step_states: alloc::vec::Vec<StepSnapshot>,
    pub(crate) final_state: BoundarySnapshot,
}
