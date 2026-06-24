use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

impl TypeIds {
    pub fn take(&mut self) -> Vec<TypeId> {
        self.hash = 0;
        self.types.clear();
        core::mem::replace(&mut self.order, Vec::new())
    }
}
