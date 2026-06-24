use crate::records::constraint_solver::ConstraintSolver;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::location::Location;

impl ConstraintSolver {
    pub fn report_error_type_error(&mut self, e: crate::records::type_error::TypeError) {
        unsafe {
            self.errors.push(e);
            let last_error = self.errors.last_mut().unwrap();
            if let Some(ref module) = self.module {
                last_error.module_name = (*module).name.clone();
            }
        }
    }
}
