use crate::records::duplicate_type_definition::DuplicateTypeDefinition;
use crate::records::error_converter::ErrorConverter;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_22(&self, e: &DuplicateTypeDefinition) -> String {
        let mut s = String::from("Redefinition of type '");
        s.push_str(e.name());
        s.push('\'');

        if let Some(previous_location) = e.previousLocation() {
            s.push_str(", previously defined at line ");
            s.push_str(&(previous_location.begin.line + 1).to_string());
        }

        s
    }
}
