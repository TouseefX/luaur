use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::follow_type::follow_type_id;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use alloc::vec::Vec;

impl ConstraintSolver {
    pub fn fill_in_discriminant_types(
        &mut self,
        constraint: *const Constraint,
        discriminant_types: &Vec<Option<TypeId>>,
    ) {
        for ty_opt in discriminant_types.iter() {
            let ty = match ty_opt {
                Some(ty) => *ty,
                None => continue,
            };

            let follow_ty = unsafe { follow_type_id(ty) };

            if self.is_blocked_type_id(follow_ty) {
                let mutable_ty = unsafe { as_mutable_type_id(follow_ty) };
                unsafe {
                    (*mutable_ty).ty = TypeVariant::Bound((*self.builtin_types).noRefineType);
                }
            }

            let constraint_loc = unsafe { (*constraint).location };
            self.unblock_type_id_location(ty, constraint_loc);
        }
    }
}
