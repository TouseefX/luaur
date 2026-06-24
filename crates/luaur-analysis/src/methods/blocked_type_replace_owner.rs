use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;

impl BlockedType {
    pub fn replace_owner(&mut self, new_owner: *const Constraint) {
        self.owner = new_owner;
    }
}
