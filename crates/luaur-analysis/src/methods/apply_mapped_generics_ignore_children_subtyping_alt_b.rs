use crate::records::apply_mapped_generics::ApplyMappedGenerics;
use crate::type_aliases::type_pack_id::TypePackId;

impl ApplyMappedGenerics {
    pub fn ignore_children_type_pack_id(&mut self, ty: TypePackId) -> bool {
        unsafe { (*ty).persistent }
    }
}
