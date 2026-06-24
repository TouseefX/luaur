use crate::records::type_cacher::TypeCacher;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

impl TypeCacher {
    pub fn visit_type_id_unknown_type(&mut self, ty: TypeId, _ut: &UnknownType) -> bool {
        self.cache(ty);
        false
    }
}
