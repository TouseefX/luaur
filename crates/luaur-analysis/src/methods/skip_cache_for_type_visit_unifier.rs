use crate::records::free_type::FreeType;
use crate::records::skip_cache_for_type::SkipCacheForType;
use crate::type_aliases::type_id::TypeId;

impl SkipCacheForType {
    pub fn visit_type_id_free_type(&mut self, _ty: TypeId, _ft: &FreeType) -> bool {
        self.result = true;
        false
    }
}
