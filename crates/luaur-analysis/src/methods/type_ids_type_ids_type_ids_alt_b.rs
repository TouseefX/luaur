use crate::records::type_ids::TypeIds;

impl TypeIds {
    pub fn drop(&mut self) {
        self.types.clear();
        self.order.clear();
    }
}
