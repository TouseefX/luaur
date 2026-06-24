use crate::records::txn_log::TxnLog;

impl TxnLog {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `TxnLog` does not implement `Clone` or `Copy`,
    /// so an explicit assignment operator is not provided.
    #[allow(dead_code)]
    pub fn operator_assign(&mut self, _other: &TxnLog) -> &mut Self {
        panic!("TxnLog copy assignment is deleted");
    }
}
