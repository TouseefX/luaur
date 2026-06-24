use crate::functions::get_mutable_level::get_mutable_level;
use crate::records::inplace_demoter::InplaceDemoter;
use crate::type_aliases::type_id::TypeId;

impl InplaceDemoter {
    pub fn demote(&mut self, ty: TypeId) -> bool {
        let level = unsafe { get_mutable_level(ty) };
        if !level.is_null() {
            if unsafe { (*level).subsumes_strict(&self.new_level) } {
                unsafe {
                    *level = self.new_level;
                }
                return true;
            }
        }
        false
    }
}
