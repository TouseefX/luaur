use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_or_pack_id::TypeOrPackId;
use alloc::boxed::Box;
use alloc::vec::Vec;

impl TxnLog {
    pub fn push_seen_type_or_pack_id_type_or_pack_id(
        &mut self,
        lhs: TypeOrPackId,
        rhs: TypeOrPackId,
    ) {
        if self.shared_seen.is_null() {
            self.shared_seen = Box::into_raw(Box::new(Vec::new()));
        }

        let sorted_pair = if lhs > rhs { (lhs, rhs) } else { (rhs, lhs) };

        unsafe {
            (*self.shared_seen).push(sorted_pair);
        }
    }
}
