use crate::enums::table_state::TableState;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::table_type::TableType;
use crate::records::txn_log::TxnLog;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;

impl TxnLog {
    pub fn get_level(&self, ty: TypeId) -> Option<TypeLevel> {
        unsafe {
            // Check FreeType
            if let Some(ftv) = get_mutable_type_id::<FreeType>(ty).as_ref() {
                return Some(ftv.level);
            }

            // Check TableType with Free or Generic state
            if let Some(ttv) = get_mutable_type_id::<TableType>(ty).as_ref() {
                if ttv.state == TableState::Free || ttv.state == TableState::Generic {
                    return Some(ttv.level);
                }
            }

            // Check FunctionType
            if let Some(ftv) = get_mutable_type_id::<FunctionType>(ty).as_ref() {
                return Some(ftv.level);
            }

            None
        }
    }
}
