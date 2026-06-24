use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack_id::TypeOrPackId;

impl TxnLog {
    #[inline]
    pub fn have_seen_type_id_type_id(&self, lhs: TypeId, rhs: TypeId) -> bool {
        self.have_seen_type_or_pack_id_type_or_pack_id(lhs as TypeOrPackId, rhs as TypeOrPackId)
    }
}
