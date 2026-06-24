use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_or_pack_id::TypeOrPackId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TxnLog {
    pub fn push_seen_type_pack_id_type_pack_id(&mut self, lhs: TypePackId, rhs: TypePackId) {
        self.push_seen_type_or_pack_id_type_or_pack_id(lhs as TypeOrPackId, rhs as TypeOrPackId);
    }
}
