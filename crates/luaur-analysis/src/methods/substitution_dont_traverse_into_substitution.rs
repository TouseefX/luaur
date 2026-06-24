use crate::records::substitution::Substitution;
use crate::type_aliases::type_id::TypeId;

impl Substitution {
    pub fn dont_traverse_into_type_id(&mut self, ty: TypeId) {
        self.no_traverse_types.insert(ty);
    }
}
