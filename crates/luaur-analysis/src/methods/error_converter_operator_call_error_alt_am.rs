use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::error_converter::ErrorConverter;
use crate::records::explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_8(&self, e: &ExplicitFunctionAnnotationRecommended) -> String {
        let recommended_return = to_string_type_id(e.recommendedReturn());
        let mut arg_annotations = String::new();

        for (arg, type_id) in e.recommendedArgs() {
            if !arg_annotations.is_empty() {
                arg_annotations.push_str(", ");
            }
            arg_annotations.push_str(arg);
            arg_annotations.push_str(": ");
            arg_annotations.push_str(&to_string_type_id(*type_id));
        }

        if arg_annotations.is_empty() {
            String::from("Consider annotating the return with ") + &recommended_return
        } else {
            String::from("Consider placing the following annotations on the arguments: ")
                + &arg_annotations
                + " or instead annotating the return as "
                + &recommended_return
        }
    }
}
