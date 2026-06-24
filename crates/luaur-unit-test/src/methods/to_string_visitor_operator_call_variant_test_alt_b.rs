impl crate::records::to_string_visitor::ToStringVisitor {
    pub fn operator_call_mut(&self, v: core::ffi::c_int) -> alloc::string::String {
        alloc::format!("{}", v)
    }
}
