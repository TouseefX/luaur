use crate::records::error_converter::ErrorConverter;
use crate::records::unapplied_type_function::UnappliedTypeFunction;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_56(&self, _e: &UnappliedTypeFunction) -> String {
        String::from("Type functions always require `<>` when referenced.")
    }
}
