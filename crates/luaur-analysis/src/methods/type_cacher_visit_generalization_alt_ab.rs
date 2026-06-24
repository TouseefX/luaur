use crate::methods::type_cacher_visit_generalization_alt_i::cacher_traverse_type_pack_id;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::bound_type_pack::BoundTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeCacher {
    /// C++ `bool TypeCacher::visit(TypePackId tp, const BoundTypePack& btp)`
    /// (Generalization.cpp:631-637).
    pub fn visit_type_pack_id_bound_type_pack(
        &mut self,
        tp: TypePackId,
        btp: &BoundTypePack,
    ) -> bool {
        cacher_traverse_type_pack_id(self, btp.boundTo);
        if self.is_uncacheable_type_pack_id(btp.boundTo) {
            self.mark_uncacheable_type_pack_id(tp);
        }
        false
    }
}
