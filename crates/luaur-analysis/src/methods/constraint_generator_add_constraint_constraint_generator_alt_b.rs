use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::type_aliases::scope_ptr_type::ScopePtr;

impl ConstraintGenerator {
    pub fn add_constraint_scope_ptr_unique_ptr_constraint(
        &mut self,
        _scope: &ScopePtr,
        c: Box<Constraint>,
    ) -> Box<Constraint> {
        let c_ptr: *mut Constraint = Box::into_raw(c);
        self.constraints.push(c_ptr);
        unsafe { Box::from_raw(*self.constraints.last().unwrap()) }
    }
}
