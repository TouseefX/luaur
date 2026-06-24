use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::skip_cache_for_type::SkipCacheForType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

impl SkipCacheForType {
    pub fn visit_type_id_table_type(&mut self, ty: TypeId, _tt: &TableType) -> bool {
        unsafe {
            if (*ty).owning_arena != self.type_arena as *mut _ {
                return false;
            }
            let ttv = get_mutable_type_id::<TableType>(ty);
            if (*ttv).bound_to.is_some()
                || (*ttv).state != crate::enums::table_state::TableState::Sealed
            {
                self.result = true;
                return false;
            }
        }
        true
    }
}
