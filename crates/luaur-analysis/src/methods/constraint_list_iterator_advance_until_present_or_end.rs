use crate::records::constraint_list::ConstraintList;
use crate::records::iterator::Iterator;
use core::ptr::NonNull;

impl Iterator {
    pub fn advance_until_present_or_end(&mut self) {
        while self.index < unsafe { self.cl.as_ref().order.len() }
            && !unsafe {
                ConstraintList::contains(
                    self.cl.as_ref(),
                    self.cl.as_ref().order[self.index].clone(),
                )
            }
        {
            self.index += 1;
        }
    }
}
