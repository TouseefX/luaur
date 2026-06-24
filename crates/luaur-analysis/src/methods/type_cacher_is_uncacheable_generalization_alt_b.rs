use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeCacher {
    pub fn is_uncacheable_type_pack_id(&self, tp: TypePackId) -> bool {
        unsafe { self.uncacheable_packs.contains(&follow_type_pack_id(tp)) }
    }
}
