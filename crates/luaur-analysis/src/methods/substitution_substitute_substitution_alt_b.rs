use crate::enums::tarjan_result::TarjanResult;
use crate::records::substitution::Substitution;
use crate::type_aliases::type_pack_id::TypePackId;

impl Substitution {
    pub fn substitute_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
        let tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

        self.base.clear_tarjan(self.base.log);

        let result = self.base.find_dirty_type_pack_id(tp);
        if result != TarjanResult::Ok {
            return None;
        }

        let new_types_clone = self.new_types.clone();
        for (old_ty, new_ty) in new_types_clone.iter() {
            if !self.base.ignore_children_type_id(*old_ty) && !self.replaced_types.contains(new_ty)
            {
                if !self.no_traverse_types.contains(new_ty) {
                    self.replace_children_type_id(*new_ty);
                }
                self.replaced_types.insert(*new_ty);
            }
        }

        let new_packs_clone = self.new_packs.clone();
        for (old_tp, new_tp) in new_packs_clone.iter() {
            if !self.base.ignore_children_type_pack_id(*old_tp)
                && !self.replaced_type_packs.contains(new_tp)
            {
                if !self.no_traverse_type_packs.contains(new_tp) {
                    self.replace_children_type_pack_id(*new_tp);
                }
                self.replaced_type_packs.insert(*new_tp);
            }
        }

        Some(self.replace_type_pack_id(tp))
    }
}
