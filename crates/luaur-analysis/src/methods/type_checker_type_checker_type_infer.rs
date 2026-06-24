//! Source: `Analysis/include/Luau/TypeInfer.h:70` (hand-ported)
use crate::records::type_checker::TypeChecker;

impl TypeChecker {
    /// C++ `TypeChecker(const TypeChecker&) = delete;` — the deleted copy ctor
    /// (TypeChecker is non-copyable); never callable in C++ either.
    pub fn type_checker_type_checker(&mut self) {
        unreachable!("C++ TypeChecker copy-ctor is `= delete` — non-copyable, no call site")
    }
}
