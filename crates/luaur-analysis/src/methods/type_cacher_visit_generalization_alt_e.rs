use crate::records::primitive_type::PrimitiveType;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_id::TypeId;

impl TypeCacher {
    pub fn visit_type_id_primitive_type(&mut self, ty: TypeId, _pt: &PrimitiveType) -> bool {
        self.cache(ty);
        false
    }
}
