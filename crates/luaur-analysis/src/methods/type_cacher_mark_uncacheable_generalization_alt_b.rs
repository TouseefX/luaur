use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeCacher {
    pub fn mark_uncacheable_type_pack_id(&mut self, tp: TypePackId) {
        let followed = unsafe { follow_type_pack_id(tp) };
        self.uncacheable_packs.insert(followed);
    }
}
