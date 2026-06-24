use crate::functions::find_table_property_respecting_meta_type_utils::find_table_property_respecting_meta_not_null_builtin_types_error_vec_type_id_string_location_bool;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn find_table_property_respecting_meta(
        &mut self,
        lhs_type: TypeId,
        name: Name,
        location: &Location,
        add_errors: bool,
    ) -> Option<TypeId> {
        let mut errors: ErrorVec = ErrorVec::new();
        let result =
            find_table_property_respecting_meta_not_null_builtin_types_error_vec_type_id_string_location_bool(
                self.builtin_types,
                &mut errors,
                lhs_type,
                name.as_str(),
                *location,
                false,
            );
        if add_errors {
            self.report_errors(&errors);
        }
        result
    }
}
