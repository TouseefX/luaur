use crate::records::txn_log::TxnLog;

impl TxnLog {
    pub fn operator_assign_mut(&mut self, _other: TxnLog) -> &mut Self {
        *self = _other;
        self
    }
}
