use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::extra_information::ExtraInformation;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::type_id::TypeId;
use alloc::format;
use alloc::string::String;
use luaur_ast::records::location::Location;

pub fn report_available_overloads(
    errors: &mut ErrorVec,
    location: Location,
    module_name: &ModuleName,
    overloads: &alloc::vec::Vec<TypeId>,
) {
    if overloads.len() <= 1 {
        return;
    }

    let mut s = String::from("Available overloads: ");
    for (i, &ty) in overloads.iter().enumerate() {
        if i > 0 {
            s.push_str(if i == overloads.len() - 1 {
                "; and "
            } else {
                "; "
            });
        }
        s.push_str(&to_string_type_id(ty));
    }

    errors.push(
        crate::records::type_error::TypeError::type_error_location_module_name_type_error_data(
            location,
            module_name.clone(),
            crate::type_aliases::type_error_data::TypeErrorData::ExtraInformation(
                ExtraInformation::new(s),
            ),
        ),
    );
}
