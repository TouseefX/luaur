use crate::records::type_ids::TypeIds;

impl TypeIds {
    pub fn get_hash(&self) -> usize {
        self.hash
    }
}
