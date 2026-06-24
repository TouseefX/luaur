use crate::functions::follow_type_pack_alt_h::follow_pack_full;
use crate::records::txn_log::TxnLog;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;

fn pending_type_pack_mapper(context: *const core::ffi::c_void, tp: TypePackId) -> TypePackId {
    let log = unsafe { &*(context as *const TxnLog) };
    let state = log.pending_type_pack_id(tp);

    if state.is_null() {
        tp
    } else {
        unsafe { &(*state).pending as *const TypePackVar }
    }
}

impl TxnLog {
    pub fn follow_type_pack_id(&self, tp: TypePackId) -> TypePackId {
        unsafe {
            follow_pack_full(
                tp,
                self as *const TxnLog as *const core::ffi::c_void,
                pending_type_pack_mapper,
            )
        }
    }
}
