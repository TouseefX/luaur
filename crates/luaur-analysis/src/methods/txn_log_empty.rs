use crate::records::txn_log::TxnLog;
use alloc::boxed::Box;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;
use std::sync::OnceLock;

impl TxnLog {
    pub fn empty() -> *const TxnLog {
        static EMPTY_LOG: OnceLock<usize> = OnceLock::new();

        *EMPTY_LOG.get_or_init(|| {
            let mut log = Box::new(TxnLog {
                type_var_changes: DenseHashMap::new(core::ptr::null()),
                type_pack_changes: DenseHashMap::new(core::ptr::null()),
                parent: core::ptr::null_mut(),
                owned_seen: Vec::new(),
                shared_seen: core::ptr::null_mut(),
                radioactive: false,
            });

            log.shared_seen = &mut log.owned_seen;
            Box::into_raw(log) as usize
        }) as *const TxnLog
    }
}
