use crate::records::error_converter::ErrorConverter;
use crate::records::multiple_nonviable_overloads::MultipleNonviableOverloads;
use alloc::string::String;
use alloc::string::ToString;

impl ErrorConverter {
    pub fn operator_call_46(&self, e: &MultipleNonviableOverloads) -> String {
        let mut result = String::from("None of the overloads for function that accept ");
        result.push_str(&e.attempted_arg_count.to_string());
        result.push_str(" arguments are compatible.");
        result
    }
}
