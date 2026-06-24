use crate::enums::unify_result::UnifyResult;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::upper_bounds::UpperBounds;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Result {
    pub unified: UnifyResult,
    pub outstanding_constraints: Vec<ConstraintV>,
    pub upper_bound_contributors: UpperBounds,
}

impl Default for Result {
    fn default() -> Self {
        Self {
            unified: UnifyResult::Ok,
            outstanding_constraints: Vec::new(),
            upper_bound_contributors: UpperBounds::new(core::ptr::null_mut()),
        }
    }
}

unsafe impl Send for Result {}
unsafe impl Sync for Result {}
