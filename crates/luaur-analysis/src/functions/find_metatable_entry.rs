use crate::functions::follow_type::follow_type_id;
use crate::functions::get_metatable_type::get_metatable_type_id_not_null_builtin_types;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::generic_error::GenericError;
use crate::records::table_type::TableType;
use crate::records::type_error::TypeError;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

pub fn find_metatable_entry(
    builtin_types: *mut BuiltinTypes,
    errors: &mut ErrorVec,
    ty: TypeId,
    entry: &str,
    location: Location,
) -> Option<TypeId> {
    let ty = unsafe { follow_type_id(ty) };

    let metatable = get_metatable_type_id_not_null_builtin_types(ty, unsafe { &*builtin_types });
    if metatable.is_none() {
        return None;
    }

    let unwrapped = unsafe { follow_type_id(metatable.unwrap()) };

    let any_type = unsafe { get_type_id::<AnyType>(unwrapped) };
    if !any_type.is_null() {
        return Some(unsafe { (*builtin_types).anyType });
    }

    let mtt = get_table_type(unwrapped);
    let mtt = match mtt {
        Some(t) => t,
        None => {
            errors.push(TypeError::type_error_location_type_error_data(
                location,
                TypeErrorData::GenericError(GenericError::new(
                    "Metatable was not a table".to_string(),
                )),
            ));
            return None;
        }
    };

    if let Some(prop_val) = mtt.props.get(entry) {
        if let Some(read_ty) = prop_val.read_ty {
            return Some(read_ty);
        } else if let Some(write_ty) = prop_val.write_ty {
            return Some(write_ty);
        }
    }

    None
}
