use crate::records::code_too_complex::CodeTooComplex;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn report_code_too_complex(&mut self, location: Location) {
        unsafe {
            let err = TypeErrorData::CodeTooComplex(CodeTooComplex { _unused: None });
            self.errors.push(crate::records::type_error::TypeError {
                location,
                module_name: (*self.module.as_ref().unwrap()).name.clone(),
                data: err.clone(),
            });
            if !self.logger.is_null() {
                (*self.logger).capture_generation_error(self.errors.last().unwrap());
            }
            self.recursion_limit_met = true;
        }
    }
}
