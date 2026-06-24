use crate::records::type_function_finder::TypeFunctionFinder;
use crate::records::type_once_visitor::TypeOnceVisitor;

impl TypeFunctionFinder {
    pub fn type_function_finder_type_function_finder() -> Self {
        TypeFunctionFinder {
            base: TypeOnceVisitor::new("TypeFunctionFinder".to_string(), true),
            ..TypeFunctionFinder::new()
        }
    }
}
