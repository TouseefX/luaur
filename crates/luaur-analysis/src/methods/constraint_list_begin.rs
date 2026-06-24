use crate::records::constraint_list::ConstraintList;
use crate::records::iterator::Iterator;
use core::ptr::NonNull;

impl ConstraintList {
    pub fn begin(&mut self) -> Iterator {
        let mut iter = Iterator {
            cl: NonNull::new(self as *mut ConstraintList as *mut ConstraintList).unwrap(),
            index: 0,
        };
        iter.advance_until_present_or_end();
        iter
    }
}
