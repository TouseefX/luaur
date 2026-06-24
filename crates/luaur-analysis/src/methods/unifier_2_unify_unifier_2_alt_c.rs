//! Source: `Analysis/src/Unifier2.cpp:149-302` — `Unifier2::unify_(TypeId, TypeId)`,
//! the core of new-solver type unification.

use crate::enums::unify_result::UnifyResult;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_irresolvable_unifier_2::is_irresolvable;
use crate::records::any_type::AnyType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter;
use crate::records::subtype_constraint::SubtypeConstraint;
use crate::records::table_type::TableType;
use crate::records::unifier_2::Unifier2;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_id::TypeId;
use luaur_common::{FFlag, FInt};

impl Unifier2 {
    pub fn unify_type_id_type_id(
        &mut self,
        mut sub_ty: TypeId,
        mut super_ty: TypeId,
    ) -> UnifyResult {
        if FInt::LuauTypeInferIterationLimit.get() > 0
            && self.iteration_count >= FInt::LuauTypeInferIterationLimit.get() as i32
        {
            return UnifyResult::TooComplex;
        }

        self.iteration_count += 1;

        // NOTE: It's a little odd that we are doing something non-exceptional for
        // the core of unification but not for occurs check, which may throw an
        // exception. It would be nice if, in the future, this were unified.
        if FFlag::LuauLimitUnificationRecursion.get() {
            // ++(*count) — mirror the C++ NonExceptionalRecursionLimiter (RecursionCounter ctor).
            self.recursion_count += 1;
            let mut nerl = NonExceptionalRecursionLimiter {
                base: unsafe { core::mem::zeroed() },
                native_stack_guard: unsafe { core::mem::zeroed() },
            };
            nerl.non_exceptional_recursion_limiter_non_exceptional_recursion_limiter(
                &mut self.recursion_count as *mut i32 as *mut core::ffi::c_int,
            );
            if !nerl.is_ok(self.recursion_limit as core::ffi::c_int) {
                return UnifyResult::TooComplex;
            }
        }

        sub_ty = unsafe { follow_type_id(sub_ty) };
        super_ty = unsafe { follow_type_id(super_ty) };

        if let Some(sub_gen) = self.generic_substitutions.find(&sub_ty) {
            let sub_gen = *sub_gen;
            return self.unify_type_id_type_id(sub_gen, super_ty);
        }

        if let Some(super_gen) = self.generic_substitutions.find(&super_ty) {
            let super_gen = *super_gen;
            return self.unify_type_id_type_id(sub_ty, super_gen);
        }

        if self.seen_type_pairings.contains(&(sub_ty, super_ty)) {
            return UnifyResult::Ok;
        }
        self.seen_type_pairings.insert((sub_ty, super_ty));

        if sub_ty == super_ty {
            return UnifyResult::Ok;
        }

        // We have potentially done some unifications while dispatching either `SubtypeConstraint` or `PackSubtypeConstraint`,
        // so rather than implementing backtracking or traversing the entire type graph multiple times, we could push
        // additional constraints as we discover blocked types along with their proper bounds.
        //
        // But we exclude these two subtyping patterns, they are tautological:
        //   - never <: *blocked*
        //   - *blocked* <: unknown
        if (is_irresolvable(sub_ty) || is_irresolvable(super_ty))
            && unsafe { get_type_id::<NeverType>(sub_ty) }.is_null()
            && unsafe { get_type_id::<UnknownType>(super_ty) }.is_null()
        {
            if !self.uninhabited_type_functions.is_null()
                && unsafe {
                    (*self.uninhabited_type_functions)
                        .contains(&(sub_ty as *const core::ffi::c_void))
                        || (*self.uninhabited_type_functions)
                            .contains(&(super_ty as *const core::ffi::c_void))
                }
            {
                return UnifyResult::Ok;
            }

            self.incomplete_subtypes
                .push(ConstraintV::Subtype(SubtypeConstraint {
                    sub_type: sub_ty,
                    super_type: super_ty,
                }));
            return UnifyResult::Ok;
        }

        let sub_free = unsafe { get_mutable_type_id::<FreeType>(sub_ty) };
        let super_free = unsafe { get_mutable_type_id::<FreeType>(super_ty) };

        if !super_free.is_null() {
            let instantiated = self.instantiate_with_bound_types(sub_ty);
            let new_lower = self.mk_union(unsafe { (*super_free).lower_bound }, instantiated);
            unsafe {
                (*super_free).lower_bound = new_lower;
            }
        }

        if !sub_free.is_null() {
            return self.unify_free_with_type(sub_ty, super_ty);
        }

        if !sub_free.is_null() || !super_free.is_null() {
            return UnifyResult::Ok;
        }

        let sub_fn = unsafe { get_type_id::<FunctionType>(sub_ty) };
        let super_fn = unsafe { get_type_id::<FunctionType>(super_ty) };
        if !sub_fn.is_null() && !super_fn.is_null() {
            return self.unify_type_id_function_type(sub_ty, unsafe { &*super_fn });
        }

        let sub_union = unsafe { get_type_id::<UnionType>(sub_ty) };
        let super_union = unsafe { get_type_id::<UnionType>(super_ty) };
        if !sub_union.is_null() {
            return self.unify_union_type_type_id(unsafe { &*sub_union }, super_ty);
        } else if !super_union.is_null() {
            return self.unify_type_id_union_type(sub_ty, unsafe { &*super_union });
        }

        let sub_intersection = unsafe { get_type_id::<IntersectionType>(sub_ty) };
        let super_intersection = unsafe { get_type_id::<IntersectionType>(super_ty) };
        if !sub_intersection.is_null() {
            return self.unify_intersection_type_type_id(unsafe { &*sub_intersection }, super_ty);
        } else if !super_intersection.is_null() {
            return self.unify_type_id_intersection_type(sub_ty, unsafe { &*super_intersection });
        }

        let sub_never = unsafe { get_type_id::<NeverType>(sub_ty) };
        let super_never = unsafe { get_type_id::<NeverType>(super_ty) };
        if !sub_never.is_null() && !super_never.is_null() {
            return UnifyResult::Ok;
        } else if !sub_never.is_null() && !super_fn.is_null() {
            // If `never` is the subtype, then we can propagate that inward.
            let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
            let never_pack = builtin_types.neverTypePack;
            let super_fn_ref = unsafe { &*super_fn };
            let arg_result =
                self.unify_type_pack_id_type_pack_id(super_fn_ref.arg_types, never_pack);
            let ret_result =
                self.unify_type_pack_id_type_pack_id(never_pack, super_fn_ref.ret_types);
            return arg_result & ret_result;
        } else if !sub_fn.is_null() && !super_never.is_null() {
            // If `never` is the supertype, then we can propagate that inward.
            let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
            let never_pack = builtin_types.neverTypePack;
            let sub_fn_ref = unsafe { &*sub_fn };
            let arg_result = self.unify_type_pack_id_type_pack_id(never_pack, sub_fn_ref.arg_types);
            let ret_result = self.unify_type_pack_id_type_pack_id(sub_fn_ref.ret_types, never_pack);
            return arg_result & ret_result;
        }

        let sub_any = unsafe { get_type_id::<AnyType>(sub_ty) };
        let super_any = unsafe { get_type_id::<AnyType>(super_ty) };

        let sub_table = unsafe { get_mutable_type_id::<TableType>(sub_ty) };
        let super_table = unsafe { get_type_id::<TableType>(super_ty) };

        if !sub_any.is_null() && !super_any.is_null() {
            return UnifyResult::Ok;
        } else if !sub_any.is_null() && !super_fn.is_null() {
            return self.unify_any_type_function_type(unsafe { &*sub_any }, unsafe { &*super_fn });
        } else if !sub_fn.is_null() && !super_any.is_null() {
            return self.unify_function_type_any_type(unsafe { &*sub_fn }, unsafe { &*super_any });
        } else if !sub_any.is_null() && !super_table.is_null() {
            return self.unify_any_type_table_type(unsafe { &*sub_any }, unsafe { &*super_table });
        } else if !sub_table.is_null() && !super_any.is_null() {
            return self.unify_table_type_any_type(unsafe { &*sub_table }, unsafe { &*super_any });
        }

        if !sub_table.is_null() && !super_table.is_null() {
            // `boundTo` works like a bound type, and therefore we'd replace it
            // with the `boundTo` and try unification again.
            //
            // However, these pointers should have been chased already by follow().
            luaur_common::macros::luau_assert::LUAU_ASSERT!(
                unsafe { (*sub_table).bound_to }.is_none()
            );
            luaur_common::macros::luau_assert::LUAU_ASSERT!(
                unsafe { (*super_table).bound_to }.is_none()
            );

            return self
                .unify_table_type_table_type(unsafe { &mut *sub_table }, unsafe { &*super_table });
        }

        let sub_metatable = unsafe { get_type_id::<MetatableType>(sub_ty) };
        let super_metatable = unsafe { get_type_id::<MetatableType>(super_ty) };
        if !sub_metatable.is_null() && !super_metatable.is_null() {
            return self.unify_metatable_type_metatable_type(unsafe { &*sub_metatable }, unsafe {
                &*super_metatable
            });
        } else if !sub_metatable.is_null() && !super_any.is_null() {
            return self
                .unify_metatable_type_any_type(unsafe { &*sub_metatable }, unsafe { &*super_any });
        } else if !sub_any.is_null() && !super_metatable.is_null() {
            return self
                .unify_any_type_metatable_type(unsafe { &*sub_any }, unsafe { &*super_metatable });
        } else if !sub_metatable.is_null() {
            // if we only have one metatable, unify with the inner table
            let inner = unsafe { (*sub_metatable).table() };
            return self.unify_type_id_type_id(inner, super_ty);
        } else if !super_metatable.is_null() {
            // if we only have one metatable, unify with the inner table
            let inner = unsafe { (*super_metatable).table() };
            return self.unify_type_id_type_id(sub_ty, inner);
        }

        let sub_negation = unsafe { get_type_id::<NegationType>(sub_ty) };
        let super_negation = unsafe { get_type_id::<NegationType>(super_ty) };
        if !sub_negation.is_null() && !super_negation.is_null() {
            return self.unify_type_id_type_id(unsafe { (*sub_negation).ty }, unsafe {
                (*super_negation).ty
            });
        }

        // The unification failed, but we're not doing type checking.
        UnifyResult::Ok
    }
}
