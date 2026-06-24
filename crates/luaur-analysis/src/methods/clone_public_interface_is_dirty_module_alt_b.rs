use crate::records::clone_public_interface::ClonePublicInterface;
use crate::type_aliases::type_pack_id::TypePackId;

impl ClonePublicInterface {
    /// `bool ClonePublicInterface::isDirty(TypePackId tp)`.
    /// Reference: `Module.cpp:146-149`.
    pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
        let module = unsafe { &*self.module };
        let owning_arena = unsafe { (*tp).owningArena };
        owning_arena == (&module.internal_types as *const _ as *mut _)
    }
}
