use crate::records::txn_log::TxnLog;

impl TxnLog {
    pub fn txn_log(&mut self) {
        self.clear();
        self.txn_log_txn_log(core::ptr::null_mut());
    }
}
