use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::error_converter::ErrorConverter;
use crate::records::type_instantiation_count_mismatch::TypeInstantiationCountMismatch;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_53(&self, e: &TypeInstantiationCountMismatch) -> String {
        LUAU_ASSERT!(
            e.providedTypes() > e.maximumTypes() || e.providedTypePacks() > e.maximumTypePacks()
        );

        let mut result = String::from("Too many type parameters passed to ");

        if let Some(function_name) = e.functionName() {
            result.push('\'');
            result.push_str(function_name);
            result.push_str("', which is typed as ");
        } else {
            result.push_str("function typed as ");
        }

        result.push_str(&to_string_type_id(e.functionType()));

        result.push_str(". Expected ");

        if e.providedTypes() > e.maximumTypes() {
            result.push_str("at most ");
            result.push_str(&e.maximumTypes().to_string());
            result.push_str(" type parameter");
            if e.maximumTypes() != 1 {
                result.push('s');
            }
            result.push_str(", but ");
            result.push_str(&e.providedTypes().to_string());
            result.push_str(" provided");

            if e.providedTypePacks() > e.maximumTypePacks() {
                result.push_str(". Also expected ");
            }
        }

        if e.providedTypePacks() > e.maximumTypePacks() {
            result.push_str("at most ");
            result.push_str(&e.maximumTypePacks().to_string());
            result.push_str(" type pack");
            if e.maximumTypePacks() != 1 {
                result.push('s');
            }
            result.push_str(", but ");
            result.push_str(&e.providedTypePacks().to_string());
            result.push_str(" provided");
        }

        result.push('.');
        result
    }
}
