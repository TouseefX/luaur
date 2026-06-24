use crate::enums::pack_field::PackField;
use crate::records::path_hash::PathHash;

impl PathHash {
    pub fn operator_call_4(&self, field: &PackField) -> usize {
        *field as usize
    }
}
