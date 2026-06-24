use crate::records::generic_type_pack::GenericTypePack;
use crate::records::txn_log::TxnLog;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn is_variadic_tail(tp: TypePackId, log: &TxnLog, include_hidden_variadics: bool) -> bool {
    if log.txn_log_is::<GenericTypePack, TypePackId>(tp) {
        return true;
    }

    if let Some(vtp) = unsafe { log.txn_log_get::<VariadicTypePack, TypePackId>(tp).as_ref() } {
        if include_hidden_variadics || !vtp.hidden {
            return true;
        }
    }

    false
}
