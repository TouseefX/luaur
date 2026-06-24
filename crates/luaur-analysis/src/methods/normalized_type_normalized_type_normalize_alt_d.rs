use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
    pub fn normalized_type_normalized_type_mut(&mut self, other: NormalizedType) {
        *self = other;
    }
}
