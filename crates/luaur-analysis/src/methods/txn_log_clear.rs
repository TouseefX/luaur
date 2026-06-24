use crate::records::txn_log::TxnLog;

impl TxnLog {
    pub fn clear(&mut self) {
        self.type_var_changes =
            luaur_common::records::dense_hash_map::DenseHashMap::new(core::ptr::null());
        self.type_pack_changes =
            luaur_common::records::dense_hash_map::DenseHashMap::new(core::ptr::null());
    }
}
