use crate::records::txn_log::TxnLog;

impl TxnLog {
    pub fn txn_log_txn_log_mut(&mut self, _other: TxnLog) -> &mut Self {
        *self = _other;
        self
    }
}
