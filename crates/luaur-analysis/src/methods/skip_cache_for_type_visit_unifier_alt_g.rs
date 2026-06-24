use crate::records::skip_cache_for_type::SkipCacheForType;
use crate::type_aliases::type_id::TypeId;

impl SkipCacheForType {
    pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
        unsafe {
            if (*ty).owning_arena != self.type_arena as *mut _ {
                return false;
            }
            if let Some(prev) = (*self.skip_cache_for_type).find(&ty) {
                if *prev {
                    self.result = true;
                    return false;
                }
            }
        }
        true
    }
}
