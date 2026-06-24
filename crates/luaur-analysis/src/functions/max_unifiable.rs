//! Source: `Analysis/include/Luau/Unifiable.h:62-68` (hand-ported)
use crate::records::type_level::TypeLevel;

/// C++ `inline TypeLevel max(const TypeLevel& a, const TypeLevel& b)`.
/// Returns the deeper/greater level: if `a` subsumes `b`, `b` is the greater.
pub fn max(a: &TypeLevel, b: &TypeLevel) -> TypeLevel {
    if a.subsumes(b) {
        *b
    } else {
        *a
    }
}
