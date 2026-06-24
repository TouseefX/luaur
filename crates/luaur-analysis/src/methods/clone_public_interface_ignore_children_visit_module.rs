use crate::records::clone_public_interface::ClonePublicInterface;
use crate::type_aliases::type_id::TypeId;

impl ClonePublicInterface {
    /// `bool ClonePublicInterface::ignoreChildrenVisit(TypeId ty)`.
    /// Reference: `Module.cpp:151-157`.
    pub fn ignore_children_visit_type_id(&mut self, ty: TypeId) -> bool {
        let module = unsafe { &*self.module };

        let owning_arena = unsafe { (*ty).owning_arena };
        if owning_arena != (&module.internal_types as *const _ as *mut _) {
            return true;
        }

        false
    }
}
