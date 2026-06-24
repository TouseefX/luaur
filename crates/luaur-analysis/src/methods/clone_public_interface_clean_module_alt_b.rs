use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::clone_public_interface::ClonePublicInterface;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generic_type_pack::GenericTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl ClonePublicInterface {
    /// `TypePackId ClonePublicInterface::clean(TypePackId tp)`.
    /// Reference: `Module.cpp:210-229`.
    pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        if self.is_new_solver() {
            if unsafe {
                !get_type_pack_id::<FreeTypePack>(tp).is_null()
                    || !get_type_pack_id::<BlockedTypePack>(tp).is_null()
            } {
                self.internal_type_escaped = true;
                return unsafe { (*self.builtin_types).errorTypePack };
            }

            let cloned_tp = self.base.clone_type_pack_id(tp);
            let gtp = unsafe { get_mutable_type_pack_id::<GenericTypePack>(cloned_tp) };
            if !gtp.is_null() {
                unsafe { (*gtp).scope = core::ptr::null_mut() };
            }
            cloned_tp
        } else {
            self.base.clone_type_pack_id(tp)
        }
    }
}
