use crate::records::inplace_demoter::InplaceDemoter;
use crate::type_aliases::type_id::TypeId;

impl InplaceDemoter {
    pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
        if unsafe { (*ty).owning_arena != self.arena } {
            return false;
        }
        self.demote(ty)
    }
}
