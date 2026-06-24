//! Source: `Analysis/src/Unifier.cpp` (PromoteTypeLevels::visit(TypePackId), L58-65)
use crate::records::promote_type_levels::PromoteTypeLevels;
use crate::records::type_arena::TypeArena;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;

impl PromoteTypeLevels {
    /// `bool visit(TypePackId tp) override` (Unifier.cpp:58)
    ///
    /// Type levels of types from other modules are already global, so we don't
    /// need to promote anything inside.
    pub fn visit_type_pack_id_unifier(&mut self, tp: TypePackId) -> bool {
        unsafe {
            let tp_var: *const TypePackVar = tp as *const TypePackVar;
            if (*tp_var).owningArena != self.type_arena as *mut TypeArena {
                return false;
            }
        }
        true
    }
}
