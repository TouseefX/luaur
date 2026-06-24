//! Source: `Analysis/src/SubtypingUnifier.cpp:83-183` — `SubtypingUnifier::dispatchOneConstraint`.

use crate::enums::occurs_check_result::OccursCheckResult;
use crate::enums::unify_result::UnifyResult;
use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::functions::emplace_type_pack::emplace_type_pack;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_2::get2;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::is_blocked_type_utils::is_blocked;
use crate::functions::occurs_check_type_utils_alt_b::occurs_check_type_pack_id_type_pack_id;
use crate::functions::simplify_intersection_simplify::simplify_intersection;
use crate::functions::simplify_union::simplify_union;
use crate::records::constraint::Constraint;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::pack_subtype_constraint::PackSubtypeConstraint;
use crate::records::subtype_constraint::SubtypeConstraint;
use crate::records::subtyping_unifier::SubtypingUnifier;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::type_aliases::constraint_v::{ConstraintV, ConstraintVMember};
use crate::type_aliases::type_pack_variant::TypePackVariant;
use crate::type_aliases::upper_bounds::UpperBounds;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl SubtypingUnifier {
    pub fn dispatch_one_constraint(
        &self,
        constraint: *const Constraint,
        cv: &ConstraintV,
        upper_bound_contributors: &mut UpperBounds,
    ) -> (UnifyResult, bool) {
        let builtin_types = self.builtin_types;
        let arena = self.arena;

        if let Some(sc) = SubtypeConstraint::get_if(cv) {
            let sub_ty = unsafe { follow_type_id(sc.sub_type) };
            let super_ty = unsafe { follow_type_id(sc.super_type) };

            LUAU_ASSERT!(self.can_be_unified(sub_ty) || self.can_be_unified(super_ty));

            if is_blocked(sub_ty) || is_blocked(super_ty) {
                return (UnifyResult::Ok, false);
            }

            let super_free_ty = unsafe { get_mutable_type_id::<FreeType>(super_ty) };
            if !super_free_ty.is_null() {
                let lower = unsafe { (*super_free_ty).lower_bound };
                let result = simplify_union(builtin_types, arena, sub_ty, lower).result;
                unsafe {
                    (*super_free_ty).lower_bound = result;
                }
            }

            let sub_free_type = unsafe { get_mutable_type_id::<FreeType>(sub_ty) };
            if !sub_free_type.is_null() {
                let upper = unsafe { (*sub_free_type).upper_bound };
                let result = simplify_intersection(builtin_types, arena, upper, super_ty).result;
                unsafe {
                    (*sub_free_type).upper_bound = result;
                }
                let location = unsafe { (*constraint).location };
                upper_bound_contributors
                    .get_or_insert(sub_ty)
                    .push((location, super_ty));
            }

            // FIXME CLI-182960: Unification shouldn't be the mechanism for adding table indexers
            let pair = get2::<TableType, TableType, _>(sub_ty, super_ty);
            if !pair.first.is_null() && unsafe { (*pair.first).indexer }.is_none() {
                let super_indexer = unsafe { (*pair.second).indexer.clone() }.unwrap();
                let sub_table = unsafe { get_mutable_type_id::<TableType>(sub_ty) };
                unsafe {
                    (*sub_table).indexer = Some(TableIndexer {
                        index_type: super_indexer.index_type,
                        index_result_type: super_indexer.index_result_type,
                        is_read_only: false,
                    });
                }
            }
        } else if let Some(psc) = PackSubtypeConstraint::get_if(cv) {
            let sub_tp = unsafe { follow_type_pack_id(psc.sub_pack) };
            let super_tp = unsafe { follow_type_pack_id(psc.super_pack) };
            // There *should* be an assertion here that either part of the constraint is
            // free, but because free type packs are replaced on-the-spot, we *may*
            // encounter conflicting constraints. One example is for:
            //
            //  local callbacks: { () -> () } = {
            //      function () end,
            //      function () end
            //  }
            //
            // We'll end up minting two sets of constraints for each lambda (as we need
            // them to be _exactly_ `()` as per the table type).
            let error_pack = unsafe { (*builtin_types).errorTypePack };

            if !unsafe { get_type_pack_id::<FreeTypePack>(sub_tp) }.is_null() {
                if FFlag::LuauOccursCheckForAllBindings.get() {
                    if OccursCheckResult::Fail
                        == occurs_check_type_pack_id_type_pack_id(sub_tp, super_tp)
                    {
                        emplace_type_pack(
                            unsafe { as_mutable_type_pack_id(sub_tp) },
                            TypePackVariant::Bound(error_pack),
                        );
                        return (UnifyResult::OccursCheckFailed, true);
                    }
                } else {
                    if OccursCheckResult::Fail == self.occurs_check_DEPRECATED(sub_tp, super_tp) {
                        emplace_type_pack(
                            unsafe { as_mutable_type_pack_id(sub_tp) },
                            TypePackVariant::Bound(error_pack),
                        );
                        return (UnifyResult::OccursCheckFailed, true);
                    }
                }
                emplace_type_pack(
                    unsafe { as_mutable_type_pack_id(sub_tp) },
                    TypePackVariant::Bound(super_tp),
                );
                return (UnifyResult::Ok, true);
            }

            if !unsafe { get_type_pack_id::<FreeTypePack>(super_tp) }.is_null() {
                if FFlag::LuauOccursCheckForAllBindings.get() {
                    if OccursCheckResult::Fail
                        == occurs_check_type_pack_id_type_pack_id(super_tp, sub_tp)
                    {
                        emplace_type_pack(
                            unsafe { as_mutable_type_pack_id(super_tp) },
                            TypePackVariant::Bound(error_pack),
                        );
                        return (UnifyResult::OccursCheckFailed, true);
                    }
                } else {
                    if OccursCheckResult::Fail == self.occurs_check_DEPRECATED(super_tp, sub_tp) {
                        emplace_type_pack(
                            unsafe { as_mutable_type_pack_id(super_tp) },
                            TypePackVariant::Bound(error_pack),
                        );
                        return (UnifyResult::OccursCheckFailed, true);
                    }
                }

                emplace_type_pack(
                    unsafe { as_mutable_type_pack_id(super_tp) },
                    TypePackVariant::Bound(sub_tp),
                );
                return (UnifyResult::Ok, true);
            }
        } else {
            LUAU_ASSERT!(false); // Unreachable, unexpected constraint in subtyping unifier.
            return (UnifyResult::Ok, false);
        }

        (UnifyResult::Ok, true)
    }
}
