use crate::records::constraint::Constraint;
use crate::records::scope::Scope;
use crate::type_aliases::constraint_v::ConstraintV;
use alloc::vec::Vec;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;

impl Constraint {
    /// C++ `Constraint::Constraint(NotNull<Scope> scope, const Location& location, ConstraintV&& c)`
    /// (Constraint.cpp:12-17).
    pub fn constraint_not_null_scope_location_constraint_v(
        scope: NonNull<Scope>,
        location: &Location,
        c: ConstraintV,
    ) -> Self {
        Constraint {
            scope: scope.as_ptr(),
            location: location.clone(),
            c,
            deprecated_dependencies: Vec::new(),
        }
    }
}
