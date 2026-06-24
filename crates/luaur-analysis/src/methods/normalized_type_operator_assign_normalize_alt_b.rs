use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
    #[allow(non_snake_case)]
    pub fn operator_assign_mut(&mut self, _other: NormalizedType) -> &mut Self {
        *self = _other;
        self
    }
}
