use crate::records::pending_type::PendingType;
use crate::records::pending_type_pack::PendingTypePack;
use crate::type_aliases::type_or_pack_id::TypeOrPackId;
use alloc::boxed::Box;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct TxnLog {
    pub(crate) type_var_changes:
        DenseHashMap<*const crate::records::r#type::Type, Box<PendingType>>,
    pub(crate) type_pack_changes:
        DenseHashMap<*const crate::records::type_pack_var::TypePackVar, Box<PendingTypePack>>,
    pub(crate) parent: *mut TxnLog,
    pub(crate) owned_seen: Vec<(TypeOrPackId, TypeOrPackId)>,
    pub(crate) shared_seen: *mut Vec<(TypeOrPackId, TypeOrPackId)>,
    pub(crate) radioactive: bool,
}
