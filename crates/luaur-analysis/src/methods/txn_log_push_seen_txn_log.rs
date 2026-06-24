use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack_id::TypeOrPackId;

impl TxnLog {
    pub fn push_seen_type_id_type_id(&mut self, lhs: TypeId, rhs: TypeId) {
        let lhs_top: TypeOrPackId = lhs as TypeOrPackId;
        let rhs_top: TypeOrPackId = rhs as TypeOrPackId;
        self.push_seen_type_or_pack_id_type_or_pack_id(lhs_top, rhs_top);
    }
}
