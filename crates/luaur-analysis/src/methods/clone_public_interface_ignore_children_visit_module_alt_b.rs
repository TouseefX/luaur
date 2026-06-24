use crate::records::clone_public_interface::ClonePublicInterface;
use crate::type_aliases::type_pack_id::TypePackId;

impl ClonePublicInterface {
    /// `bool ClonePublicInterface::ignoreChildrenVisit(TypePackId tp)`.
    /// Reference: `Module.cpp:159-165`.
    pub fn ignore_children_visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
        let module = unsafe { &*self.module };

        let owning_arena = unsafe { (*tp).owningArena };
        if owning_arena != (&module.internal_types as *const _ as *mut _) {
            return true;
        }

        false
    }
}
