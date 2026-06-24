use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::skip_cache_for_type::SkipCacheForType;
use crate::type_aliases::type_pack_id::TypePackId;

impl SkipCacheForType {
    pub fn visit_type_pack_id_blocked_type_pack(
        &mut self,
        _tp: TypePackId,
        _btp: &BlockedTypePack,
    ) -> bool {
        self.result = true;
        false
    }
}
