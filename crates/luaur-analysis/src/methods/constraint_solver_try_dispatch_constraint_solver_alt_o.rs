use crate::enums::polarity::Polarity;
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::extend_type_pack::extend_type_pack;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::fresh_type::fresh_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::track_interior_free_type::track_interior_free_type;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::unpack_constraint::UnpackConstraint;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn try_dispatch_unpack_constraint_not_null_constraint(
        &mut self,
        c: &UnpackConstraint,
        constraint: *const Constraint,
    ) -> bool {
        let source_pack = unsafe { follow_type_pack_id(c.source_pack) };

        if self.is_blocked_type_pack_id(source_pack) {
            return self.block_type_pack_id_not_null_constraint(source_pack, constraint);
        }

        let src_pack = unsafe {
            extend_type_pack(
                &mut *self.arena,
                self.builtin_types,
                source_pack,
                c.result_pack.len(),
                alloc::vec::Vec::new(),
            )
        };

        let mut i = 0;
        while i < c.result_pack.len() {
            if i >= src_pack.head.len() {
                break;
            }

            let src_ty = unsafe { follow_type_id(src_pack.head[i]) };
            let result_ty = unsafe { follow_type_id(c.result_pack[i]) };

            if unsafe { !get_type_id::<BlockedType>(result_ty).is_null() } {
                LUAU_ASSERT!(can_mutate_type_id(result_ty, constraint));

                if unsafe { follow_type_id(src_ty) } == result_ty {
                    let fresh_ty = unsafe {
                        fresh_type(
                            &mut *self.arena,
                            &*self.builtin_types,
                            (*constraint).scope,
                            Polarity::Positive,
                        )
                    };
                    track_interior_free_type(unsafe { (*constraint).scope }, fresh_ty);

                    if FFlag::LuauConstraintGraph.get() {
                        self.bind_not_null_constraint_type_id_type_id(
                            constraint, result_ty, fresh_ty,
                        );
                    } else {
                        self.deprecate_d_shift_references(result_ty, fresh_ty);
                        unsafe {
                            (*as_mutable_type_id(result_ty)).ty = TypeVariant::Bound(fresh_ty);
                        }
                    }
                } else {
                    self.bind_not_null_constraint_type_id_type_id(constraint, result_ty, src_ty);
                }
            } else {
                self.constraint_solver_unify(constraint, src_ty, result_ty);
            }

            if !FFlag::LuauConstraintGraph.get() {
                self.unblock_type_id_location(result_ty, unsafe { (*constraint).location });
            }

            i += 1;
        }

        while i < c.result_pack.len() {
            let result_ty = unsafe { follow_type_id(c.result_pack[i]) };
            LUAU_ASSERT!(can_mutate_type_id(result_ty, constraint));

            if unsafe {
                !get_type_id::<BlockedType>(result_ty).is_null()
                    || !get_type_id::<PendingExpansionType>(result_ty).is_null()
            } {
                self.bind_not_null_constraint_type_id_type_id(constraint, result_ty, unsafe {
                    (*self.builtin_types).nilType
                });
            }

            i += 1;
        }

        true
    }
}

fn can_mutate_type_id(ty: TypeId, constraint: *const Constraint) -> bool {
    let blocked = unsafe { get_type_id::<BlockedType>(ty) };
    if !blocked.is_null() {
        let owner = unsafe { (*blocked).getOwner() };
        LUAU_ASSERT!(!owner.is_null());
        return owner == constraint;
    }

    true
}
