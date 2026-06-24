use crate::records::txn_log::TxnLog;
use alloc::boxed::Box;
use alloc::vec::Vec;

impl TxnLog {
    pub fn txn_log_txn_log(&mut self, _parent: *mut TxnLog) {
        self.parent = _parent;

        if !_parent.is_null() {
            self.shared_seen = unsafe { (*_parent).shared_seen };
        } else {
            self.shared_seen = Box::into_raw(Box::new(Vec::new()));
        }
    }
}
