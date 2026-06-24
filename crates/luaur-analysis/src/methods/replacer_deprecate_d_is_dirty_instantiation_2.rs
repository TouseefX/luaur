use crate::records::replacer_deprecated::ReplacerDeprecated;
use crate::type_aliases::type_id::TypeId;

impl ReplacerDeprecated {
    pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
        self.replacements.find(&ty).is_some()
    }
}
