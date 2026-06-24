use crate::records::generic_type_pack::GenericTypePack;
use crate::records::skip_cache_for_type::SkipCacheForType;
use crate::type_aliases::type_pack_id::TypePackId;

impl SkipCacheForType {
    pub fn visit_type_pack_id_generic_type_pack(
        &mut self,
        _tp: TypePackId,
        _gtp: &GenericTypePack,
    ) -> bool {
        self.result = true;
        false
    }
}
