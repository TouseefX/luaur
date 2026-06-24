use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn add_constraint_scope_ptr_location_constraint_v(
        &mut self,
        scope: &ScopePtr,
        location: Location,
        cv: ConstraintV,
    ) -> *mut Constraint {
        let c = Box::new(Constraint {
            scope: scope.as_ref() as *const _ as *mut _,
            location,
            c: cv,
            deprecated_dependencies: alloc::vec::Vec::new(),
        });
        let c_ptr = Box::into_raw(c);
        self.constraints.push(c_ptr);
        c_ptr
    }
}
