//! Source: `Analysis/include/Luau/Constraint.h:345` (hand-ported)
impl crate::records::constraint::Constraint {
    /// C++ `Constraint(const Constraint&) = delete;` — the deleted copy ctor
    /// (the real ctor is `Constraint::constraint_not_null_scope_location_constraint_v`);
    /// never callable in C++ either.
    pub fn constraint_constraint() {
        unreachable!("C++ Constraint copy-ctor is `= delete` — non-copyable, no call site")
    }
}
