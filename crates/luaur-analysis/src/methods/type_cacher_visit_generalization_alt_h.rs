use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_id::TypeId;

impl TypeCacher {
    pub fn visit_type_id_pending_expansion_type(
        &mut self,
        ty: TypeId,
        _pet: &PendingExpansionType,
    ) -> bool {
        self.mark_uncacheable_type_id(ty);
        false
    }
}
