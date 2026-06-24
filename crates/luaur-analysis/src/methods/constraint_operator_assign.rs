impl crate::records::constraint::Constraint {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `Constraint` does not implement `Clone` or `Copy`,
    /// so an explicit assignment operator is not provided.
    #[allow(dead_code)]
    pub fn constraint_operator_assign(
        &mut self,
        _other: &crate::records::constraint::Constraint,
    ) -> &mut crate::records::constraint::Constraint {
        panic!("Constraint is not assignable");
    }
}
