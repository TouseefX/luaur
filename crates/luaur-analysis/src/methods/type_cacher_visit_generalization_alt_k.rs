use crate::functions::follow_type::follow_type_id;
use crate::methods::type_cacher_visit_generalization_alt_i::cacher_traverse_type_id;
use crate::records::metatable_type::MetatableType;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_id::TypeId;

impl TypeCacher {
    pub fn visit_type_id_metatable_type(&mut self, ty: TypeId, mtv: &MetatableType) -> bool {
        let tbl = unsafe { follow_type_id(mtv.table()) };
        let mt = unsafe { follow_type_id(mtv.metatable()) };
        cacher_traverse_type_id(self, tbl);
        cacher_traverse_type_id(self, mt);
        if self.is_uncacheable_type_id(tbl) || self.is_uncacheable_type_id(mt) {
            self.mark_uncacheable_type_id(ty);
        } else {
            self.cache(ty);
        }
        false
    }
}
