use crate::enums::value_context::ValueContext;
use crate::functions::find_table_property_respecting_meta_type_utils_alt_b::find_table_property_respecting_meta;
use crate::records::builtin_types::BuiltinTypes;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

pub fn find_table_property_respecting_meta_not_null_builtin_types_error_vec_type_id_string_location_bool(
    builtin_types: *mut BuiltinTypes,
    errors: &mut ErrorVec,
    ty: TypeId,
    name: &str,
    location: Location,
    use_new_solver: bool,
) -> Option<TypeId> {
    find_table_property_respecting_meta(
        builtin_types,
        errors,
        ty,
        name,
        ValueContext::RValue,
        location,
        use_new_solver,
    )
}
