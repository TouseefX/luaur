use crate::records::function_type::FunctionType;
use crate::records::txn_log::TxnLog;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct OverloadErrorEntry {
    pub(crate) log: TxnLog,
    pub(crate) errors: ErrorVec,
    pub(crate) arguments: alloc::vec::Vec<TypeId>,
    pub(crate) fn_ty: *const FunctionType,
}
