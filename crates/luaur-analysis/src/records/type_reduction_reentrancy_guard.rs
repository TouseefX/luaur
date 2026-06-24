use crate::records::unifier_shared_state::UnifierSharedState;

#[derive(Debug)]
pub struct TypeReductionReentrancyGuard {
    pub(crate) shared_state: *mut UnifierSharedState,
}
