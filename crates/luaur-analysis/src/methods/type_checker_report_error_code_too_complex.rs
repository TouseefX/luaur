use crate::records::code_too_complex::CodeTooComplex;
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_noinline::LUAU_NOINLINE;

impl TypeChecker {
    pub fn report_error_code_too_complex(&mut self, location: &Location) {
        let error = TypeError::type_error_location_type_error_data(
            *location,
            CodeTooComplex::default().into(),
        );
        self.report_error_type_error(&error);
    }
}
