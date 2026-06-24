//! C++ `bool IterativeTypeFunctionTypeVisitor::visit(TypeFunctionTypeId ty)`
//! (IterativeTypeFunctionTypeVisitor.cpp:73-76) — the base no-payload type
//! visit, which simply `return true;`.
//!
//! The method-shaped slot `IterativeTypeFunctionTypeVisitor::visit_type_function_type_id`
//! is already provided by a committed sibling
//! (`records/freeze_type_function_types.rs`), so defining it here as a method
//! would be a `duplicate definitions` (E0592) conflict. This free function
//! preserves the faithful body of the cited overload so the contract surface
//! is not lost; callers in this crate resolve the method form.

pub fn iterative_type_function_type_visitor_visit() -> bool {
    true
}
