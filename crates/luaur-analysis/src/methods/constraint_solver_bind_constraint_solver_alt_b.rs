//! `void ConstraintSolver::bind(NotNull<const Constraint> constraint, TypePackId tp, TypePackId boundTo)`
//! (`Analysis/src/ConstraintSolver.cpp:982-1001`, hand-ported faithfully).

use crate::enums::occurs_check_result::OccursCheckResult;
use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::occurs_check_type_utils_alt_b::occurs_check_type_pack_id_type_pack_id;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::internal_error::InternalError;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn bind_not_null_constraint_type_pack_id_type_pack_id(
        &mut self,
        constraint: *const Constraint,
        tp: TypePackId,
        bound_to: TypePackId,
    ) {
        LUAU_ASSERT!(unsafe {
            !get_type_pack_id::<BlockedTypePack>(tp).is_null()
                || !get_type_pack_id::<FreeTypePack>(tp).is_null()
        });
        LUAU_ASSERT!(can_mutate_type_pack_id(tp, constraint));

        let bound_to = unsafe { follow_type_pack_id(bound_to) };
        LUAU_ASSERT!(tp != bound_to);

        let location = unsafe { (*constraint).location };

        if FFlag::LuauOccursCheckForAllBindings.get()
            && occurs_check_type_pack_id_type_pack_id(tp, bound_to) == OccursCheckResult::Fail
        {
            self.report_error_type_error_data_location(
                InternalError {
                    message: alloc::string::String::from("Attempted to create a type pack cycle"),
                }
                .into(),
                &location,
            );
            let mutable_tp = unsafe { as_mutable_type_pack_id(tp) };
            let mut err_arg = unsafe { (*self.builtin_types).errorTypePack };
            crate::methods::unifiable_bound_type_pack_id_emplace_type_pack_bound_type_pack::emplace_type_pack(
                mutable_tp,
                &mut err_arg,
            );
        } else {
            let mutable_tp = unsafe { as_mutable_type_pack_id(tp) };
            let mut bound_arg = bound_to;
            crate::methods::unifiable_bound_type_pack_id_emplace_type_pack_bound_type_pack::emplace_type_pack(
                mutable_tp,
                &mut bound_arg,
            );
        }

        self.unblock_type_pack_id_location(tp, location);
    }
}

// C++ `[[maybe_unused]] static bool canMutate(TypePackId tp, NotNull<const Constraint> constraint)`
// (`Analysis/src/ConstraintSolver.cpp:101-111`), used only in asserts.
fn can_mutate_type_pack_id(tp: TypePackId, constraint: *const Constraint) -> bool {
    let blocked = unsafe { get_type_pack_id::<BlockedTypePack>(tp) };
    if !blocked.is_null() {
        let owner = unsafe { (*blocked).owner };
        LUAU_ASSERT!(!owner.is_null());
        return owner as *const Constraint == constraint;
    }
    true
}
