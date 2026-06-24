use crate::records::skip_cache_for_type::SkipCacheForType;
use crate::type_aliases::type_pack_id::TypePackId;

impl SkipCacheForType {
    pub fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
        unsafe {
            if (*tp).owningArena != self.type_arena as *mut _ {
                return false;
            }
        }
        true
    }
}
