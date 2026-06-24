use crate::records::replacer_deprecated::ReplacerDeprecated;
use crate::type_aliases::type_pack_id::TypePackId;

impl ReplacerDeprecated {
    pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
        self.replacement_packs.find(&tp).is_some()
    }
}
