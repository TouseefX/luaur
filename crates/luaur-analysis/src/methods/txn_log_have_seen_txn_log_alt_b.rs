use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_or_pack_id::TypeOrPackId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TxnLog {
    #[inline]
    pub fn have_seen_type_pack_id_type_pack_id(&self, lhs: TypePackId, rhs: TypePackId) -> bool {
        self.have_seen_type_or_pack_id_type_or_pack_id(lhs as TypeOrPackId, rhs as TypeOrPackId)
    }
}
