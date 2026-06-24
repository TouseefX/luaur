use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn is_blocked_txn_log_type_pack_id(log: &TxnLog, tp: TypePackId) -> bool {
    let tp = log.follow_type_pack_id(tp);
    !unsafe { crate::functions::get_type_pack::get::<BlockedTypePack>(tp) }.is_null()
}
