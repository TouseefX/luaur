use crate::records::blocked_type::BlockedType;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_id::TypeId;

impl TypeCacher {
    pub fn visit_type_id_blocked_type(&mut self, ty: TypeId, _bt: &BlockedType) -> bool {
        self.mark_uncacheable_type_id(ty);
        false
    }
}
