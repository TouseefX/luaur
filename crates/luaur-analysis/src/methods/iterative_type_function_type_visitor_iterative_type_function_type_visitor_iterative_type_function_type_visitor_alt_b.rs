//! Source: `Analysis/include/Luau/IterativeTypeFunctionTypeVisitor.h:41` (hand-ported)
use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;

impl IterativeTypeFunctionTypeVisitor {
    /// C++ `IterativeTypeFunctionTypeVisitor(const IterativeTypeFunctionTypeVisitor&) = delete;` —
    /// the deleted copy ctor (the real ctors are the `string` / `string,bool` /
    /// `string,SeenSet,bool` variants); never callable in C++ either.
    pub fn iterative_type_function_type_visitor_iterative_type_function_type_visitor(&mut self) {
        unreachable!("C++ IterativeTypeFunctionTypeVisitor copy-ctor is `= delete` — non-copyable, no call site")
    }
}
