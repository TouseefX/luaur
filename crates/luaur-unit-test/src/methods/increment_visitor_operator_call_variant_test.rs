impl crate::records::increment_visitor::IncrementVisitor {
    pub fn operator_call(&self, v: &mut alloc::string::String) {
        v.push_str("1");
    }
}
