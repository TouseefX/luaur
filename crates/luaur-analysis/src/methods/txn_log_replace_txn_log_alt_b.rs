use crate::records::pending_type_pack::PendingTypePack;
use crate::records::txn_log::TxnLog;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;

impl TxnLog {
    pub fn replace_type_pack_id_t<T>(
        &mut self,
        tp: TypePackId,
        replacement: T,
    ) -> *mut PendingTypePack
    where
        T: Into<TypePackVar>,
    {
        self.replace_type_pack_id_type_pack_var(tp, replacement.into())
    }
}
