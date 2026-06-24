use crate::records::constraint_list::ConstraintList;
use core::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Iterator {
    pub(crate) cl: NonNull<ConstraintList>,
    pub(crate) index: usize,
}
