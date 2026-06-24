use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::constraint_v::ConstraintV;

impl SubtypingResult {
    pub fn with_assumed_constraint(&mut self, constraint: ConstraintV) -> &mut Self {
        self.assumed_constraints.push(constraint);
        self
    }
}
