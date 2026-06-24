use crate::functions::follow_type::follow_type_id;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_id::TypeId;

impl TypeCacher {
    pub fn mark_uncacheable_type_id(&mut self, ty: TypeId) {
        let followed = unsafe { follow_type_id(ty) };
        self.uncacheable.insert(followed);
    }
}
