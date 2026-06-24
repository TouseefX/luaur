//! `void ConstraintSolver::bind(NotNull<const Constraint> constraint, TypeId ty, TypeId boundTo)`
//! (`Analysis/src/ConstraintSolver.cpp:938-980`, hand-ported faithfully).

use crate::enums::polarity::Polarity;
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::fresh_type::fresh_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::track_interior_free_type::track_interior_free_type;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::free_type::FreeType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn bind_not_null_constraint_type_id_type_id(
        &mut self,
        constraint: *const Constraint,
        ty: TypeId,
        bound_to: TypeId,
    ) {
        LUAU_ASSERT!(unsafe {
            !get_type_id::<BlockedType>(ty).is_null()
                || !get_type_id::<FreeType>(ty).is_null()
                || !get_type_id::<PendingExpansionType>(ty).is_null()
        });
        LUAU_ASSERT!(can_mutate_type_id(ty, constraint));

        let bound_to = unsafe { follow_type_id(bound_to) };
        let scope = unsafe { (*constraint).scope };
        let location = unsafe { (*constraint).location };

        if FFlag::LuauOccursCheckForAllBindings.get() {
            // This follow shouldn't be needed, but if for some reason we end up
            // with a bound type, we want to also follow it when doing this
            // occurence check.
            if unsafe { follow_type_id(ty) } == bound_to {
                let fresh_ty = fresh_type(
                    unsafe { &mut *self.arena },
                    unsafe { &*self.builtin_types },
                    scope,
                    Polarity::Mixed,
                );
                let mutable_ty = unsafe { as_mutable_type_id(ty) };
                let mut fresh_arg = fresh_ty;
                crate::methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type(
                    unsafe { &mut *mutable_ty },
                    &mut fresh_arg,
                );
                track_interior_free_type(scope, fresh_ty);
                self.unblock_type_id_location(ty, location);
                return;
            }
        } else if unsafe { !get_type_id::<BlockedType>(ty).is_null() } && ty == bound_to {
            // DEPRECATED_emplace<FreeType>(constraint, ty, scope, neverType, unknownType, Polarity::Mixed)
            // FIXME?  Is this the right polarity?
            let free_ty = FreeType::free_type_scope_type_id_type_id_polarity(
                scope,
                unsafe { (*self.builtin_types).neverType },
                unsafe { (*self.builtin_types).unknownType },
                Polarity::Mixed,
            );
            let mutable_ty = unsafe { as_mutable_type_id(ty) };
            unsafe {
                (*mutable_ty).ty = TypeVariant::Free(free_ty);
            }
            self.unblock_type_id_location(ty, location);
            track_interior_free_type(scope, ty);
            return;
        }

        let mutable_ty = unsafe { as_mutable_type_id(ty) };
        let mut bound_arg = bound_to;
        crate::methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type(
            unsafe { &mut *mutable_ty },
            &mut bound_arg,
        );

        if !FFlag::LuauConstraintGraph.get() {
            // `unblock` will "shift references" under the hood.
            self.deprecate_d_shift_references(ty, bound_to);
        }

        self.unblock_type_id_location(ty, location);
    }
}

// C++ `[[maybe_unused]] static bool canMutate(TypeId ty, NotNull<const Constraint> constraint)`
// (`Analysis/src/ConstraintSolver.cpp:88-98`), used only in asserts.
fn can_mutate_type_id(ty: TypeId, constraint: *const Constraint) -> bool {
    let blocked = unsafe { get_type_id::<BlockedType>(ty) };
    if !blocked.is_null() {
        let owner = unsafe { (*blocked).getOwner() };
        LUAU_ASSERT!(!owner.is_null());
        return owner == constraint;
    }
    true
}
