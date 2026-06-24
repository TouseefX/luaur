use crate::records::constraint::Constraint;

#[derive(Debug, Clone)]
pub struct BlockedType {
    pub(crate) index: i32,
    /// The constraint that is intended to unblock this type. Other constraints
    /// should block on this constraint if present.
    pub(crate) owner: *const Constraint,
}

#[allow(non_snake_case)]
impl BlockedType {
    pub fn getOwner(&self) -> *const Constraint {
        self.owner
    }

    pub fn setOwner(&mut self, new_owner: *const Constraint) {
        self.owner = new_owner;
    }

    pub fn replaceOwner(&mut self, new_owner: *const Constraint) {
        self.owner = new_owner;
    }
}

impl Default for BlockedType {
    fn default() -> Self {
        Self {
            index: 0,
            owner: core::ptr::null(),
        }
    }
}

impl BlockedType {
    pub fn blocked_type(&mut self) {
        self.index = 0;
        self.owner = core::ptr::null();
    }
}
