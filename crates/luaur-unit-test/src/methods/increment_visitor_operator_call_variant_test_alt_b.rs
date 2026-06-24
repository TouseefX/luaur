use crate::records::increment_visitor::IncrementVisitor;

impl IncrementVisitor {
    pub fn operator_call_mut(&self, v: &mut core::ffi::c_int) {
        *v += 1;
    }
}
