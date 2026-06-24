use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;

impl BlockedType {
    pub fn get_owner(&self) -> *const Constraint {
        self.owner
    }
}
