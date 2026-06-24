use crate::records::blocked_type::BlockedType;
use crate::records::blocked_type_finder::BlockedTypeFinder;
use crate::type_aliases::type_id::TypeId;

impl BlockedTypeFinder {
    pub fn visit_type_id_blocked_type(&mut self, ty: TypeId, _blocked: &BlockedType) -> bool {
        self.blocked = Some(ty);
        false
    }
}
