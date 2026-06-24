use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::subtyping_reasonings::SubtypingReasonings;

impl SubtypingResult {
    pub fn reasoning(&self) -> &SubtypingReasonings {
        &self.reasoning
    }
}
