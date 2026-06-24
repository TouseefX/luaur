use crate::enums::value_context::ValueContext;
use crate::functions::find_metatable_entry::find_metatable_entry;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::function_type::FunctionType;
use crate::records::generic_error::GenericError;
use crate::records::table_type::TableType;
use crate::records::type_error::TypeError;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

pub use find_table_property_respecting_meta_not_null_builtin_types_error_vec_type_id_string_value_context_location_bool as find_table_property_respecting_meta;

pub fn find_table_property_respecting_meta_not_null_builtin_types_error_vec_type_id_string_value_context_location_bool(
    builtin_types: *mut BuiltinTypes,
    errors: &mut ErrorVec,
    ty: TypeId,
    name: &str,
    context: ValueContext,
    location: Location,
    use_new_solver: bool,
) -> Option<TypeId> {
    unsafe {
        let any_type = get_type_id::<crate::records::any_type::AnyType>(ty);
        if !any_type.is_null() {
            return Some(ty);
        }

        let table_type = get_table_type(ty);
        if let Some(tt) = table_type {
            if let Some(prop) = tt.props.get(name) {
                match context {
                    ValueContext::RValue => return prop.read_ty,
                    ValueContext::LValue => return prop.write_ty,
                }
            }
        }

        let mut mt_index = find_metatable_entry(builtin_types, errors, ty, "__index", location);
        let mut count = 0;

        while let Some(index) = mt_index {
            if count >= 100 {
                return None;
            }
            count += 1;

            let index = follow_type_id(index);

            if let Some(itt) = get_table_type(index) {
                if let Some(fit) = itt.props.get(name) {
                    if use_new_solver {
                        match context {
                            ValueContext::RValue => return fit.read_ty,
                            ValueContext::LValue => return fit.write_ty,
                        }
                    } else {
                        return fit.read_ty;
                    }
                }
            } else {
                let itf = get_type_id::<FunctionType>(index);
                if !itf.is_null() {
                    let r = crate::functions::first::first(
                        follow_type_pack_id((*itf).ret_types),
                        false,
                    );
                    if let Some(r) = r {
                        return Some(r);
                    } else {
                        return Some((*builtin_types).nilType);
                    }
                }

                if !get_type_id::<crate::records::any_type::AnyType>(index).is_null() {
                    return Some((*builtin_types).anyType);
                }

                let type_str =
                    crate::functions::to_string_to_string_alt_c::to_string_type_id(index);
                errors.push(TypeError::type_error_location_type_error_data(
                    location,
                    TypeErrorData::GenericError(GenericError::new(format!(
                        "__index should either be a function or table. Got {}",
                        type_str
                    ))),
                ));
            }

            mt_index = find_metatable_entry(
                builtin_types,
                errors,
                mt_index.unwrap(),
                "__index",
                location,
            );
        }

        None
    }
}
