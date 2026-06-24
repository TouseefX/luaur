use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;

impl TypeCacher {
    pub fn visit_type_id_error_type(&mut self, ty: TypeId, _et: &ErrorType) -> bool {
        self.cache(ty);
        false
    }
}
