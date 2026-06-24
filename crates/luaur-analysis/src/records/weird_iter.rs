#[derive(Debug, Clone)]
pub struct WeirdIter {
    pub(crate) pack_id: crate::type_aliases::type_pack_id::TypePackId,
    pub(crate) log: *mut crate::records::txn_log::TxnLog,
    pub(crate) pack: *mut crate::records::type_pack::TypePack,
    pub(crate) index: usize,
    pub(crate) growing: bool,
    pub(crate) level: crate::records::type_level::TypeLevel,
    pub(crate) scope: *mut crate::records::scope::Scope,
}
