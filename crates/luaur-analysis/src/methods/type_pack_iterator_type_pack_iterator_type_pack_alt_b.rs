use crate::records::txn_log::TxnLog;
use crate::records::type_pack_iterator::TypePackIterator;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypePackIterator {
    pub fn type_pack_iterator_type_pack_id(&mut self, _type_pack: TypePackId) {
        self.type_pack_iterator_type_pack_id_txn_log(_type_pack, TxnLog::empty() as *const TxnLog);
    }
}
