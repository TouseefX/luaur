use crate::functions::find_table_property_respecting_meta_type_utils::find_table_property_respecting_meta_not_null_builtin_types_error_vec_type_id_string_location_bool;
use crate::records::unifier::Unifier;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::type_id::TypeId;

impl Unifier {
    pub fn unifier_find_table_property_respecting_meta(
        &mut self,
        lhs: TypeId,
        name: Name,
    ) -> Option<TypeId> {
        find_table_property_respecting_meta_not_null_builtin_types_error_vec_type_id_string_location_bool(
            self.builtin_types,
            &mut self.errors,
            lhs,
            &name,
            self.location,
            false,
        )
    }
}
