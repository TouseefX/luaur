use crate::records::type_ids::TypeIds;

impl TypeIds {
    pub fn type_ids_type_ids_mut(&mut self) -> Self {
        core::mem::replace(self, Self::type_ids())
    }
}
