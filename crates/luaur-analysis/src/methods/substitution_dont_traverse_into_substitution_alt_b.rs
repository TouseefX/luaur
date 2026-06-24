use crate::records::substitution::Substitution;
use crate::type_aliases::type_pack_id::TypePackId;

impl Substitution {
    pub fn dont_traverse_into_type_pack_id(&mut self, tp: TypePackId) {
        self.no_traverse_type_packs.insert(tp);
    }
}
