use crate::records::error_converter::ErrorConverter;
use crate::records::generic_type_pack_count_mismatch::GenericTypePackCountMismatch;
use alloc::string::String;
use alloc::string::ToString;

impl ErrorConverter {
    pub fn operator_call_11(&self, e: &GenericTypePackCountMismatch) -> String {
        let mut result =
            String::from("Different number of generic type pack parameters: subtype had ");
        result.push_str(&e.subTyGenericPackCount.to_string());
        result.push_str(", supertype had ");
        result.push_str(&e.superTyGenericPackCount.to_string());
        result.push('.');
        result
    }
}
