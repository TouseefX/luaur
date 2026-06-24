use crate::enums::table_state::TableState;
use crate::records::anyification::Anyification;
use crate::records::free_type::FreeType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

impl Anyification {
    pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
        unsafe {
            if (*ty).persistent {
                return false;
            }

            let log = self.base.base.log;

            let ttv = (*log).txn_log_get_mutable::<TableType, TypeId>(ty);
            if !ttv.is_null() {
                return (*ttv).state == TableState::Free || (*ttv).state == TableState::Unsealed;
            }

            let ftv = (*log).txn_log_get_mutable::<FreeType, TypeId>(ty);
            if !ftv.is_null() {
                return true;
            }

            false
        }
    }
}
