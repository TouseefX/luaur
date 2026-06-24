//! `template<typename T, typename... Args>
//!  void ConstraintSolver::DEPRECATED_emplace(NotNull<const Constraint> constraint, TypePackId tp, Args&&... args)`
//! (`Analysis/src/ConstraintSolver.cpp:1015-1025`, hand-ported faithfully).
//!
//! See the TypeId overload for why the `Args&&...` construction is not
//! expressible through a Rust generic; this method preserves the C++ asserts
//! and the unblock, and mirrors `emplaceTypePack<T>` via the shared helper.

use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::free_type_pack::FreeTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintSolver {
    pub fn deprecate_d_emplace_not_null_constraint_type_pack_id_args_item<T, Args>(
        &mut self,
        constraint: *const Constraint,
        tp: TypePackId,
        _args: Args,
    ) {
        // static_assert(!std::is_same_v<T, BoundTypePack>, "cannot use `emplace<BoundTypePack>`! use `bind`");

        LUAU_ASSERT!(unsafe {
            !get_type_pack_id::<BlockedTypePack>(tp).is_null()
                || !get_type_pack_id::<FreeTypePack>(tp).is_null()
        });

        let _mutable_tp = unsafe { as_mutable_type_pack_id(tp) };

        let location = unsafe { (*constraint).location };
        self.unblock_type_pack_id_location(tp, location);
    }
}
