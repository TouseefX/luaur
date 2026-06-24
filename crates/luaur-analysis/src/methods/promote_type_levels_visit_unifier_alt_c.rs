use crate::records::free_type::FreeType;
use crate::records::promote_type_levels::PromoteTypeLevels;
use crate::type_aliases::type_id::TypeId;

impl PromoteTypeLevels {
    pub fn visit_type_id_free_type(&mut self, ty: TypeId, _ft: &FreeType) -> bool {
        unsafe {
            if !(*self.log).txn_log_is::<FreeType, TypeId>(ty) {
                return true;
            }
            let ft = (*self.log).txn_log_get_mutable::<FreeType, TypeId>(ty);
            self.promote(ty, ft, (*ft).level);
        }
        true
    }
}
