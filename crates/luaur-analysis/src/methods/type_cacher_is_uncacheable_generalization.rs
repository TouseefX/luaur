use crate::functions::follow_type::follow_type_id;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_id::TypeId;

impl TypeCacher {
    pub fn is_uncacheable_type_id(&self, ty: TypeId) -> bool {
        unsafe { self.uncacheable.contains(&follow_type_id(ty)) }
    }
}
