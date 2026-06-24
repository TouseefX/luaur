use alloc::string::String;

use crate::records::failed_to_compile::FailedToCompile;
use crate::records::runtime_error::RuntimeError;
use crate::records::type_function_missing::TypeFunctionMissing;
use crate::records::unsupported_type::UnsupportedType;
use crate::records::unsupported_type_pack::UnsupportedTypePack;

use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::functions::to_string_to_string_alt_d::to_string_type_pack_id;

pub struct TypeFunctionErrorConverter {}

impl TypeFunctionErrorConverter {
    pub fn operator_unsupported_type(&self, e: &UnsupportedType) -> String {
        format!(
            "Type functions do not currently support types of the form '{}'",
            to_string_type_id(e.r#type)
        )
    }

    pub fn operator_unsupported_type_pack(&self, e: &UnsupportedTypePack) -> String {
        format!(
            "Type functions do not currently support types of the form '{}'",
            to_string_type_pack_id(e.pack)
        )
    }

    pub fn operator_runtime_error(&self, e: &RuntimeError) -> String {
        e.message().to_owned()
    }

    pub fn operator_failed_to_compile(&self, e: &FailedToCompile) -> String {
        format!(
            "'{}' type function failed to compile with error message: {}",
            e.functionName().to_owned(),
            e.compileError().to_owned()
        )
    }

    pub fn operator_type_function_missing(&self, e: &TypeFunctionMissing) -> String {
        format!(
            "Could not find '{}' type function in the global scope",
            e.functionName().to_owned()
        )
    }
}
