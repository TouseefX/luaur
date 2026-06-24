use alloc::string::String;

use crate::records::cannot_extend_table::CannotExtendTable;
use crate::records::duplicate_type_definition::DuplicateTypeDefinition;
use crate::records::unknown_property::UnknownProperty;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InvalidNameChecker {
    invalid_name: String,
}

impl InvalidNameChecker {
    pub fn new() -> Self {
        Self {
            invalid_name: "%error-id%".to_owned(),
        }
    }

    pub fn new_with_invalid_name(invalid_name: String) -> Self {
        Self { invalid_name }
    }
}

impl InvalidNameChecker {
    pub fn operator_unknown_property(&self, e: &UnknownProperty) -> bool {
        e.key() == self.invalid_name
    }

    pub fn operator_cannot_extend_table(&self, e: &CannotExtendTable) -> bool {
        e.prop() == self.invalid_name
    }

    pub fn operator_duplicate_type_definition(&self, e: &DuplicateTypeDefinition) -> bool {
        e.name() == self.invalid_name
    }

    pub fn operator_fallback<T>(&self, _other: &T) -> bool {
        false
    }
}

unsafe impl Send for InvalidNameChecker {}
unsafe impl Sync for InvalidNameChecker {}
