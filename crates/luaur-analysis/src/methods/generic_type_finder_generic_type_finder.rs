use crate::records::generic_type_finder::GenericTypeFinder;
use crate::records::type_once_visitor::TypeOnceVisitor;

impl GenericTypeFinder {
    pub fn generic_type_finder() -> Self {
        GenericTypeFinder {
            base: TypeOnceVisitor::new("GenericTypeFinder".to_string(), true),
            found: false,
        }
    }
}
