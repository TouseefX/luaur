use crate::functions::is_variadic_type_pack_alt_b::is_variadic_type_pack_id_txn_log;
use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn is_variadic(tp: TypePackId) -> bool {
    is_variadic_type_pack_id_txn_log(tp, unsafe { &*TxnLog::empty() })
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use is_variadic as is_variadic_type_pack_id_type_pack_id;
