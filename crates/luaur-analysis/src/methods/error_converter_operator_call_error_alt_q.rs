use crate::records::error_converter::ErrorConverter;
use crate::records::unification_too_complex::UnificationTooComplex;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_41(&self, _: &UnificationTooComplex) -> String {
        String::from("Internal error: Code is too complex to typecheck! Consider adding type annotations around this area")
    }
}
