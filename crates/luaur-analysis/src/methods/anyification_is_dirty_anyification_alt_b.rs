use crate::records::anyification::Anyification;
use crate::records::free_type_pack::FreeTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl Anyification {
    /// `bool Anyification::isDirty(TypePackId tp)` (Anyification.cpp:54-62).
    pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
        if unsafe { (*tp).persistent } {
            return false;
        }

        // C++: `if (log->getMutable<FreeTypePack>(tp)) return true; else return false;`
        let log = self.base.base.log;
        let ftp = unsafe { (*log).txn_log_get_mutable::<FreeTypePack, TypePackId>(tp) };
        !ftp.is_null()
    }
}
