use crate::records::blocked_type::BlockedType;
use crate::records::skip_cache_for_type::SkipCacheForType;
use crate::type_aliases::type_id::TypeId;

impl SkipCacheForType {
    pub fn visit_type_id_blocked_type(&mut self, _ty: TypeId, _bt: &BlockedType) -> bool {
        self.result = true;
        false
    }
}
