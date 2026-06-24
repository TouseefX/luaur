use crate::functions::get_constraint::get_constraint;
use crate::records::blocked_type::BlockedType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_id::TypeId;

pub fn is_blocked_txn_log_type_id(log: &TxnLog, ty: TypeId) -> bool {
    let ty = log.follow_type_id(ty);
    !unsafe { crate::functions::get_type_alt_j::get::<BlockedType>(ty) }.is_null()
        || !unsafe { crate::functions::get_type_alt_j::get::<PendingExpansionType>(ty) }.is_null()
}
