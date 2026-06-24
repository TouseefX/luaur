use crate::records::substitution::Substitution;
use crate::type_aliases::type_pack_id::TypePackId;

impl Substitution {
    pub fn replace_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        let tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

        if let Some(prev_tp) = self.new_packs.find(&tp) {
            *prev_tp
        } else {
            tp
        }
    }
}
