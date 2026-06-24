use crate::functions::follow_type::follow_type_id;
use crate::records::constraint_solver::ConstraintSolver;
use crate::type_aliases::type_id::TypeId;

impl ConstraintSolver {
    pub fn has_unresolved_constraints(&mut self, ty: TypeId) -> bool {
        if luaur_common::FFlag::LuauConstraintGraph.get() {
            let ty = unsafe { follow_type_id(ty) };
            unsafe {
                (*self.cgraph).has_unsolved_dependencies(
                    crate::type_aliases::constraint_vertex::ConstraintVertex::V0(ty),
                )
            }
        } else {
            let ty = unsafe { follow_type_id(ty) };
            if let Some(set) = self.deprecated_type_to_constraint_set.get(&ty) {
                !set.is_empty()
            } else {
                false
            }
        }
    }
}
