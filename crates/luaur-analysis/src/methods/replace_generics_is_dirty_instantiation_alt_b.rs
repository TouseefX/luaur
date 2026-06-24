use crate::records::replace_generics::ReplaceGenerics;
use crate::type_aliases::type_pack_id::TypePackId;

impl ReplaceGenerics {
    pub fn is_dirty_type_pack_id(&self, _tp: TypePackId) -> bool {
        self.generic_packs.iter().any(|&tp| tp == _tp)
    }
}
