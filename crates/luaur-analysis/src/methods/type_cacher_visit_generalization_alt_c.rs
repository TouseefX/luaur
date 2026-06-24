use crate::records::generic_type::GenericType;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_id::TypeId;

impl TypeCacher {
    pub fn visit_type_id_generic_type(&mut self, ty: TypeId, _gt: &GenericType) -> bool {
        self.cache(ty);
        false
    }
}
