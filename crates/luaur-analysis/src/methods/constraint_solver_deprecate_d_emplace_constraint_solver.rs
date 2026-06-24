//! `template<typename T, typename... Args>
//!  void ConstraintSolver::DEPRECATED_emplace(NotNull<const Constraint> constraint, TypeId ty, Args&&... args)`
//! (`Analysis/src/ConstraintSolver.cpp:1003-1013`, hand-ported faithfully).
//!
//! In C++ the body is `emplaceType<T>(asMutable(ty), args...)` followed by an
//! unblock. The variant-construction (`Args&&...`) is not expressible through a
//! Rust generic; callers that need a concrete `T` construct the variant inline
//! and assign it (see e.g. `bind`'s `TypeVariant::Free(...)` branch). This
//! method preserves the C++ asserts and the unblock; the `emplaceType<T>`
//! reinterpret is mirrored via the shared `emplace_type` helper.

use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::emplace_type::emplace_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::free_type::FreeType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintSolver {
    pub fn deprecate_d_emplace_not_null_constraint_type_id_args_item<T, Args>(
        &mut self,
        constraint: *const Constraint,
        ty: TypeId,
        _args: Args,
    ) {
        // static_assert(!std::is_same_v<T, BoundType>, "cannot use `emplace<BoundType>`! use `bind`");

        LUAU_ASSERT!(unsafe {
            !get_type_id::<BlockedType>(ty).is_null()
                || !get_type_id::<FreeType>(ty).is_null()
                || !get_type_id::<PendingExpansionType>(ty).is_null()
        });

        let mutable_ty = unsafe { as_mutable_type_id(ty) };
        let _ = unsafe { emplace_type::<T>(mutable_ty) };

        let location = unsafe { (*constraint).location };
        self.unblock_type_id_location(ty, location);
    }
}
