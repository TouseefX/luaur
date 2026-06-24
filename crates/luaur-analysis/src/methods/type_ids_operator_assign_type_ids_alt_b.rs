use crate::records::type_ids::TypeIds;

impl TypeIds {
    pub fn operator_assign_mut(&mut self, _rhs: TypeIds) -> &mut Self {
        *self = _rhs;
        self
    }
}
