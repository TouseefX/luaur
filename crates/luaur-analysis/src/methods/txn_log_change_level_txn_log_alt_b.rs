use crate::functions::get_mutable_txn_log_alt_c::get_mutable_pending_type_pack;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::pending_type_pack::PendingTypePack;
use crate::records::txn_log::TxnLog;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TxnLog {
    pub fn change_level_type_pack_id_type_level(
        &mut self,
        tp: TypePackId,
        new_level: TypeLevel,
    ) -> *mut PendingTypePack {
        LUAU_ASSERT!(self.txn_log_is::<FreeTypePack, TypePackId>(tp));

        let new_tp = self.queue_type_pack_id(tp);
        unsafe {
            let ftp = get_mutable_pending_type_pack::<FreeTypePack>(new_tp);
            if !ftp.is_null() {
                (*ftp).level = new_level;
            }
        }

        new_tp
    }
}
