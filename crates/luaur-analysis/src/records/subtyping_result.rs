use crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy;
use crate::records::generic_bounds_mismatch::GenericBoundsMismatch;
use crate::records::type_error::TypeError;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::subtyping_reasonings::SubtypingReasonings;

#[derive(Debug, Clone)]
pub struct SubtypingResult {
    pub(crate) is_subtype: bool,
    pub(crate) normalization_too_complex: bool,
    pub(crate) is_cacheable: bool,
    pub(crate) is_error_suppressing: bool,
    pub(crate) errors: ErrorVec,
    pub(crate) reasoning: SubtypingReasonings,
    pub(crate) assumed_constraints: alloc::vec::Vec<ConstraintV>,
    pub(crate) generic_bounds_mismatches: alloc::vec::Vec<GenericBoundsMismatch>,
}
