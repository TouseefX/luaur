use crate::functions::to_string_to_string_alt_d::to_string_type_pack_id;
use crate::records::type_function_error_converter::TypeFunctionErrorConverter;
use crate::records::unsupported_type_pack::UnsupportedTypePack;
use alloc::string::String;
use luaur_common::functions::format::format;

impl TypeFunctionErrorConverter {
    pub fn operator_call_5(&self, e: &UnsupportedTypePack) -> String {
        let pack_str = to_string_type_pack_id(e.pack);
        format(format_args!(
            "Type functions do not currently support types of the form '{}'",
            pack_str
        ))
    }
}
