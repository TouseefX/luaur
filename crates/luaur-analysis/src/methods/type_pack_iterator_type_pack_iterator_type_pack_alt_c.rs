use crate::records::txn_log::TxnLog;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_iterator::TypePackIterator;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypePackIterator {
    pub fn type_pack_iterator_type_pack_id_txn_log(
        &mut self,
        type_pack: TypePackId,
        log: *const TxnLog,
    ) {
        self.currentTypePack = unsafe { (*log).follow_type_pack_id(type_pack) };
        self.tp = unsafe { (*log).txn_log_get::<TypePack, TypePackId>(self.currentTypePack) };
        self.currentIndex = 0;
        self.log = log;

        while !self.tp.is_null() && unsafe { (*self.tp).head.is_empty() } {
            self.currentTypePack = if let Some(tail) = unsafe { (*self.tp).tail } {
                unsafe { (*log).follow_type_pack_id(tail) }
            } else {
                core::ptr::null()
            };

            self.tp = if !self.currentTypePack.is_null() {
                unsafe { (*log).txn_log_get_mutable::<TypePack, TypePackId>(self.currentTypePack) }
            } else {
                core::ptr::null_mut()
            };
        }
    }
}
