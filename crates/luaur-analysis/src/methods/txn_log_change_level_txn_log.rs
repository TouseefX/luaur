use crate::enums::table_state::TableState;
use crate::functions::get_mutable_txn_log::get_mutable_pending_type;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::pending_type::PendingType;
use crate::records::table_type::TableType;
use crate::records::txn_log::TxnLog;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TxnLog {
    pub fn change_level_type_id_type_level(
        &mut self,
        ty: TypeId,
        new_level: TypeLevel,
    ) -> *mut PendingType {
        LUAU_ASSERT!(
            self.txn_log_is::<FreeType, TypeId>(ty)
                || self.txn_log_is::<TableType, TypeId>(ty)
                || self.txn_log_is::<FunctionType, TypeId>(ty)
        );

        let new_ty = self.queue_type_id(ty);
        unsafe {
            let ftv = get_mutable_pending_type::<FreeType>(new_ty);
            if !ftv.is_null() {
                (*ftv).level = new_level;
            } else {
                let ttv = get_mutable_pending_type::<TableType>(new_ty);
                if !ttv.is_null() {
                    LUAU_ASSERT!(
                        (*ttv).state == TableState::Free || (*ttv).state == TableState::Generic
                    );
                    (*ttv).level = new_level;
                } else {
                    let ftv = get_mutable_pending_type::<FunctionType>(new_ty);
                    if !ftv.is_null() {
                        (*ftv).level = new_level;
                    }
                }
            }
        }

        new_ty
    }
}
