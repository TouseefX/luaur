use crate::records::function_type::FunctionType;
use crate::records::promote_type_levels::PromoteTypeLevels;
use crate::type_aliases::type_id::TypeId;

impl PromoteTypeLevels {
    pub fn visit_type_id_function_type(&mut self, ty: TypeId, _ft: &FunctionType) -> bool {
        unsafe {
            if (*ty).owning_arena != self.type_arena as *mut _ {
                return false;
            }
            if !(*self.log).txn_log_is::<FunctionType, TypeId>(ty) {
                return true;
            }
            let ft = (*self.log).txn_log_get_mutable::<FunctionType, TypeId>(ty);
            self.promote(ty, ft, (*ft).level);
        }
        true
    }
}
