use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::type_function_error_converter::TypeFunctionErrorConverter;
use crate::records::unsupported_type::UnsupportedType;
use alloc::string::String;
use luaur_common::functions::format::format;

impl TypeFunctionErrorConverter {
    pub fn operator_call_4(&self, e: &UnsupportedType) -> String {
        let ty = to_string_type_id(e.r#type);
        format(format_args!(
            "Type functions do not currently support types of the form '{}'",
            ty
        ))
    }
}
