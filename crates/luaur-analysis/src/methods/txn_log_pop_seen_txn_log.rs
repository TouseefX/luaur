use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack_id::TypeOrPackId;

impl TxnLog {
    pub fn pop_seen_type_id_type_id(&mut self, lhs: TypeId, rhs: TypeId) {
        self.pop_seen_type_or_pack_id_type_or_pack_id(lhs as TypeOrPackId, rhs as TypeOrPackId);
    }
}
