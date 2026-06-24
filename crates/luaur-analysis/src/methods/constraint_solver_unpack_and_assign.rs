use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::unpack_constraint::UnpackConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use core::ptr::NonNull;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintSolver {
    pub fn unpack_and_assign(
        &mut self,
        dest_types: alloc::vec::Vec<TypeId>,
        src_types: TypePackId,
        constraint: NonNull<Constraint>,
    ) -> NonNull<Constraint> {
        let c = self.push_constraint(
            NonNull::new(unsafe { (*constraint.as_ptr()).scope }).unwrap(),
            unsafe { (*constraint.as_ptr()).location },
            ConstraintV::Unpack(UnpackConstraint {
                result_pack: dest_types.clone(),
                source_pack: src_types,
            }),
        );

        for t in dest_types {
            let bt = unsafe { get_mutable_type_id::<BlockedType>(t) };
            LUAU_ASSERT!(!bt.is_null());
            unsafe { (*bt).replace_owner(c.as_ptr()) };
        }

        c
    }
}
