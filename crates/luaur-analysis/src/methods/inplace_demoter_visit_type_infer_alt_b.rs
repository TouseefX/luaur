//! @interface-stub
use crate::records::free_type_pack::FreeTypePack;
use crate::records::inplace_demoter::InplaceDemoter;
use crate::type_aliases::type_pack_id::TypePackId;

impl InplaceDemoter {
    pub fn visit_type_pack_id_free_type_pack(
        &mut self,
        tp: TypePackId,
        ftp_ref: &FreeTypePack,
    ) -> bool {
        unsafe {
            if (*tp).owningArena != self.arena {
                return false;
            }

            let ftp = ftp_ref as *const FreeTypePack as *mut FreeTypePack;
            if (*ftp).level.subsumes_strict(&self.new_level) {
                (*ftp).level = self.new_level;
                return true;
            }
        }

        false
    }
}
