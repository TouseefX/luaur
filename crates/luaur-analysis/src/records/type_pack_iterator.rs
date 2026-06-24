use crate::records::txn_log::TxnLog;
use crate::records::type_pack::TypePack;
use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct TypePackIterator {
    pub(crate) currentTypePack: TypePackId,
    pub(crate) tailCycleCheck: TypePackId,
    pub(crate) tp: *const TypePack,
    pub(crate) currentIndex: usize,
    pub(crate) log: *const TxnLog,
}
