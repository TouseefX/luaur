use crate::records::contains_refinable_type::ContainsRefinableType;
use crate::records::type_once_visitor::TypeOnceVisitor;

impl ContainsRefinableType {
    pub fn contains_refinable_type() -> Self {
        let mut visitor = ContainsRefinableType {
            base: TypeOnceVisitor::new("ContainsRefinableType".to_string(), true),
            found: false,
        };

        visitor
    }
}
