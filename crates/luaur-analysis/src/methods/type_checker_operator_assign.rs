//! Source: `Analysis/include/Luau/TypeInfer.h:71` (hand-ported)
use crate::records::type_checker::TypeChecker;

impl TypeChecker {
    /// C++ `TypeChecker& operator=(const TypeChecker&) = delete;` — the deleted
    /// copy-assignment (TypeChecker is non-copyable); never callable in C++ either.
    pub fn type_checker_operator_assign(&mut self, _other: &TypeChecker) {
        unreachable!("C++ TypeChecker copy-assign is `= delete` — non-copyable, no call site")
    }
}
