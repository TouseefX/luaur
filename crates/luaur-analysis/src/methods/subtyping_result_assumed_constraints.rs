use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::constraint_v::ConstraintV;
use alloc::vec::Vec;

impl SubtypingResult {
    pub fn assumed_constraints(&self) -> &Vec<ConstraintV> {
        &self.assumed_constraints
    }
}
