use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::records::unscoped_generic_finder::UnscopedGenericFinder;

impl UnscopedGenericFinder {
    pub fn unscoped_generic_finder() -> Self {
        UnscopedGenericFinder {
            base: TypeOnceVisitor::new("UnscopedGenericFinder".to_string(), true),
            scope_gen_tys: Vec::new(),
            scope_gen_tps: Vec::new(),
            found_unscoped: false,
        }
    }
}
