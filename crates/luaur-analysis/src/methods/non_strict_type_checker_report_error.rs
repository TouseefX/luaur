use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::location::Location;

impl NonStrictTypeChecker {
    pub fn report_error(&mut self, data: TypeErrorData, location: &Location) {
        unsafe {
            (*self.module).errors.push(crate::records::type_error::TypeError::type_error_location_module_name_type_error_data(
                *location,
                (*self.module).name.clone(),
                data,
            ));
        }
    }
}
