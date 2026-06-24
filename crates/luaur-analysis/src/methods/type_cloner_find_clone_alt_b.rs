use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::type_cloner::TypeCloner;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeCloner {
    pub fn find_type_pack_id(&self, tp: TypePackId) -> Option<TypePackId> {
        let mut tp = unsafe { follow_type_pack_id(tp) };

        if let Some(it) = unsafe { (*self.packs).get(&tp) } {
            return Some(*it);
        } else if unsafe { (*tp).persistent } && tp != self.force_tp {
            return Some(tp);
        }

        None
    }
}
