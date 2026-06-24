use crate::functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut;
use crate::records::cannot_assign_to_never::CannotAssignToNever;
use crate::records::error_converter::ErrorConverter;
use crate::records::to_string_options::ToStringOptions;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_3(&self, e: &CannotAssignToNever) -> String {
        let mut opts = ToStringOptions::default();
        let rhs_type_str = to_string_type_id_to_string_options_mut(e.rhsType(), opts);
        let mut result = String::from("Cannot assign a value of type ")
            + &rhs_type_str
            + " to a field of type never";

        if e.reason() == crate::enums::reason::Reason::PropertyNarrowed {
            if !e.cause().is_empty() {
                result.push_str(
                    "\ncaused by the property being given the following incompatible types:\n",
                );
                for ty in e.cause() {
                    let mut opts = ToStringOptions::default();
                    let ty_str = to_string_type_id_to_string_options_mut(*ty, opts);
                    result.push_str("    ");
                    result.push_str(&ty_str);
                    result.push('\n');
                }
                result.push_str(
                    "There are no values that could safely satisfy all of these types at once.",
                );
            }
        }

        result
    }
}
