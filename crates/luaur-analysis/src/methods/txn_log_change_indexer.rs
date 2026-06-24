use crate::records::pending_type::PendingType;
use crate::records::table_indexer::TableIndexer;
use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TxnLog {
    pub fn change_indexer(
        &mut self,
        ty: TypeId,
        indexer: Option<TableIndexer>,
    ) -> *mut PendingType {
        // SAFETY: We assume get<TableType>(ty) is valid per the C++ assertion
        // and that queue_type_id returns a valid PendingType pointer
        let new_ty = self.queue_type_id(ty);

        // SAFETY: get_mutable_pending_type is the Rust analog of getMutable<TableType>
        // We assume the type is a TableType as per the C++ assertion
        unsafe {
            let table_type = crate::functions::get_mutable_txn_log::get_mutable_pending_type::<
                crate::records::table_type::TableType,
            >(new_ty);
            if !table_type.is_null() {
                (*table_type).indexer = indexer;
            }
        }

        new_ty
    }
}
