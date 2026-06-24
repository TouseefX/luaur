use crate::enums::type_field::TypeField;
use crate::records::path_hash::PathHash;

impl PathHash {
    pub fn operator_call_9(&self, field: &TypeField) -> usize {
        *field as usize
    }
}
