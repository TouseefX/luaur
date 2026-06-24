use crate::records::constraint_generator::ConstraintGenerator;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn report_error(&mut self, location: Location, err: TypeErrorData) {
        unsafe {
            self.errors.push(crate::records::type_error::TypeError {
                location,
                module_name: (*self.module.as_ref().unwrap()).name.clone(),
                data: err.clone(),
            });
            if !self.logger.is_null() {
                (*self.logger).capture_generation_error(self.errors.last().unwrap());
            }
        }
    }
}
