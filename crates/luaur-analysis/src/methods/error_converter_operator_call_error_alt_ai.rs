use crate::records::error_converter::ErrorConverter;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_48(&self, _error: &NormalizationTooComplex) -> String {
        String::from(
            "Code is too complex to typecheck! Consider simplifying the code around this area",
        )
    }
}
