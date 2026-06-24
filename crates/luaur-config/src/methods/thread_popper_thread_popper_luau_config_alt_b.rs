use crate::records::thread_popper::ThreadPopper;

impl ThreadPopper {
    pub fn thread_popper_thread_popper(other: &ThreadPopper) -> Self {
        ThreadPopper { L: other.L }
    }
}
