use crate::records::anyification::Anyification;
use crate::type_aliases::type_pack_id::TypePackId;

impl Anyification {
    pub fn ignore_children_type_pack_id(&mut self, ty: TypePackId) -> bool {
        unsafe { (*ty).persistent }
    }
}
