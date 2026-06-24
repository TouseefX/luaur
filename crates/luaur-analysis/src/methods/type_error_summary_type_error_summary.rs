use crate::records::type_error_summary::TypeErrorSummary;
use luaur_ast::records::location::Location;

impl TypeErrorSummary {
    pub fn type_error_summary_type_error_summary(
        location: Location,
        module_name: crate::type_aliases::module_name_type::ModuleName,
        code: i32,
    ) -> Self {
        Self {
            location,
            module_name,
            code,
        }
    }
}
