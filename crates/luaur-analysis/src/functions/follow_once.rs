use crate::records::table_type::TableType;
use crate::records::txn_log::TxnLog;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::type_id::TypeId;

pub fn follow_once(log: &mut TxnLog, ty: TypeId) -> TypeId {
    if let Some(bound) = unsafe { log.txn_log_get::<BoundType, TypeId>(ty).as_ref() } {
        return bound.boundTo;
    }

    if let Some(tt) = unsafe { log.txn_log_get::<TableType, TypeId>(ty).as_ref() } {
        return tt.bound_to.unwrap_or(core::ptr::null());
    }

    core::ptr::null()
}
