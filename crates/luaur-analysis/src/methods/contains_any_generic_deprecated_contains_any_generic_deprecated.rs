use crate::records::contains_any_generic_deprecated::ContainsAnyGenericDeprecated;
use crate::records::type_once_visitor::TypeOnceVisitor;

impl ContainsAnyGenericDeprecated {
    pub fn contains_any_generic_deprecated() -> Self {
        ContainsAnyGenericDeprecated {
            base: TypeOnceVisitor::new("ContainsAnyGeneric".to_string(), true),
            found: false,
        }
    }
}
