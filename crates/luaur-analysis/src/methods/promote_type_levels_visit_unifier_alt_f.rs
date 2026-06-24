use crate::records::free_type_pack::FreeTypePack;
use crate::records::promote_type_levels::PromoteTypeLevels;
use crate::type_aliases::type_pack_id::TypePackId;

impl PromoteTypeLevels {
    pub fn visit_type_pack_id_free_type_pack(
        &mut self,
        tp: TypePackId,
        _ftp: &FreeTypePack,
    ) -> bool {
        unsafe {
            if !(*self.log).txn_log_is::<FreeTypePack, TypePackId>(tp) {
                return true;
            }
            let ftp = (*self.log).txn_log_get_mutable::<FreeTypePack, TypePackId>(tp);
            self.promote_pack(tp, ftp, (*ftp).level);
        }
        true
    }
}
