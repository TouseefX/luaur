use crate::records::pending_type_pack::PendingTypePack;
use crate::records::txn_log::TxnLog;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;

impl TxnLog {
    pub fn replace_type_pack_id_type_pack_var(
        &mut self,
        tp: TypePackId,
        replacement: TypePackVar,
    ) -> *mut PendingTypePack {
        let new_tp = self.queue_type_pack_id(tp);
        unsafe {
            (*new_tp).pending.reassign(&replacement);
        }
        new_tp
    }
}
