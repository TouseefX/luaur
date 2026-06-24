use crate::records::r#type::Type;

impl Type {
    pub fn type_item_type_item(&mut self, _rhs: &Type) {
        // The C++ source is `Type(const Type&) = default;`, which is the copy constructor.
        // In Rust, this maps to `Clone::clone`, but the method signature in the schedule
        // requires `&mut self, &Type` and returning nothing.
        // Since the type is `#[derive(Clone)]`, we can simply clone into self.
        // However, the schedule's interface for this method (and its siblings) expects
        // a "constructor-like" body that initializes `self` from a source.
        // Given the design card and the fact that `Type` is `#[derive(Clone)]`, the
        // copy constructor behavior is a simple clone. We implement it as a self-assignment
        // via clone to match the semantics of the C++ default copy constructor.
        *self = _rhs.clone();
    }
}
