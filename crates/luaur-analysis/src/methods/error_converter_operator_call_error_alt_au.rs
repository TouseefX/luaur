use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::error_converter::ErrorConverter;
use crate::records::unexpected_type_in_subtyping::UnexpectedTypeInSubtyping;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_58(&self, e: &UnexpectedTypeInSubtyping) -> String {
        let ty = to_string_type_id(e.ty);
        String::from("Encountered an unexpected type in subtyping: ") + &ty
    }
}
