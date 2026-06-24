use crate::records::type_error::TypeError;
use crate::records::unifier::Unifier;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::location::Location;

impl Unifier {
    pub fn report_error_location_type_error_data(
        &mut self,
        location: Location,
        data: TypeErrorData,
    ) {
        let err = TypeError::type_error_location_type_error_data(location, data);
        self.errors.push(err);
        self.failure = true;
    }
}
