use crate::enums::table_state::TableState;
use crate::records::promote_type_levels::PromoteTypeLevels;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

impl PromoteTypeLevels {
    pub fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
        unsafe {
            if (*ty).owning_arena != self.type_arena as *mut _ {
                return false;
            }

            // Unifier.cpp:99-100 — only Free/Generic table levels are promoted.
            if ttv.state != TableState::Free && ttv.state != TableState::Generic {
                return true;
            }

            if !(*self.log).txn_log_is::<TableType, TypeId>(ty) {
                return true;
            }
            let ttv_mut = (*self.log).txn_log_get_mutable::<TableType, TypeId>(ty);
            self.promote(ty, ttv_mut, (*ttv_mut).level);
        }
        true
    }
}
