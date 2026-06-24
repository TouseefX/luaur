use crate::records::txn_log::TxnLog;

impl TxnLog {
    pub fn txn_log_txn_log_mut_2(&mut self, _other: &TxnLog) -> &mut Self {
        unimplemented!("TxnLog copy assignment is deleted in C++");
    }
}
