use crate::records::constraint_list::ConstraintList;
use crate::records::iterator::Iterator;

impl ConstraintList {
    pub fn end(&mut self) -> Iterator {
        Iterator {
            cl: core::ptr::NonNull::new(self as *mut ConstraintList).unwrap(),
            index: self.order.len(),
        }
    }
}
