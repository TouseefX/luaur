//! Source: `Analysis/src/Unifier.cpp` (PromoteTypeLevels::visit(TypeId), L49-56)
use crate::records::promote_type_levels::PromoteTypeLevels;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;

impl PromoteTypeLevels {
    /// `bool visit(TypeId ty) override` (Unifier.cpp:49)
    ///
    /// Type levels of types from other modules are already global, so we don't
    /// need to promote anything inside.
    pub fn visit_type_id_unifier(&mut self, ty: TypeId) -> bool {
        unsafe {
            if (*ty).owning_arena != self.type_arena as *mut TypeArena {
                return false;
            }
        }
        true
    }
}
