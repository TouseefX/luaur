use crate::records::type_level::TypeLevel;

pub fn min(a: &TypeLevel, b: &TypeLevel) -> TypeLevel {
    if a.subsumes(b) {
        *a
    } else {
        *b
    }
}
