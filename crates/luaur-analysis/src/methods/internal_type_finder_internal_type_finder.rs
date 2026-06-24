use crate::records::internal_type_finder::InternalTypeFinder;
use crate::records::type_once_visitor::TypeOnceVisitor;

impl InternalTypeFinder {
    pub fn internal_type_finder() -> Self {
        InternalTypeFinder {
            base: TypeOnceVisitor::new("InternalTypeFinder".to_string(), true),
        }
    }
}
