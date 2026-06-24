use crate::records::pending_type::PendingType;
use crate::records::r#type::Type;
use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_id::TypeId;

impl TxnLog {
    pub fn replace_type_id_type_item(
        &mut self,
        _ty: TypeId,
        _replacement: Type,
    ) -> *mut PendingType {
        let new_ty = self.queue_type_id(_ty);
        unsafe {
            (*new_ty).pending.reassign(&_replacement);
        }
        new_ty
    }
}
