use crate::records::txn_log::TxnLog;
use alloc::boxed::Box;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl TxnLog {
    pub fn new() -> Self {
        Self {
            type_var_changes: DenseHashMap::new(core::ptr::null()),
            type_pack_changes: DenseHashMap::new(core::ptr::null()),
            parent: core::ptr::null_mut(),
            owned_seen: Vec::new(),
            shared_seen: Box::into_raw(Box::new(Vec::new())),
            radioactive: false,
        }
    }
}
