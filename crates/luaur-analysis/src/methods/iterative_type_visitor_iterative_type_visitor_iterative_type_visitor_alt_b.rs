use crate::records::iterative_type_visitor::IterativeTypeVisitor;

impl IterativeTypeVisitor {
    pub fn iterative_type_visitor_iterative_type_visitor_destructor(&mut self) {
        // The C++ destructor is default-generated and does not perform custom cleanup.
        // In Rust, the fields (Vec, String, etc.) will be dropped automatically
        // when the IterativeTypeVisitor instance goes out of scope.
    }
}
