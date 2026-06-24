//! Source: `Analysis/src/Unifier2.cpp:316-405` — `Unifier2::unifyFreeWithType`.
//!
//! If superTy is a function and subTy already has a potentially-compatible
//! function in its upper bound, we assume that the function is not overloaded
//! and attempt to combine superTy into subTy's existing function bound.

use crate::enums::unify_result::UnifyResult;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::unifier_2::Unifier2;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl Unifier2 {
    pub fn unify_free_with_type(&mut self, sub_ty: TypeId, super_ty: TypeId) -> UnifyResult {
        let sub_free = unsafe { get_mutable_type_id::<FreeType>(sub_ty) };
        LUAU_ASSERT!(!sub_free.is_null());

        let upper_bound = unsafe { follow_type_id((*sub_free).upper_bound) };

        if !unsafe { get_type_id::<FunctionType>(upper_bound) }.is_null() {
            let existing_upper = unsafe { (*sub_free).upper_bound };
            return self.unify_type_id_type_id(existing_upper, super_ty);
        }

        // When superTy is a union or intersection, propagate subTy as a lower bound into any
        // free-type members. Without this, `freeA <: 'T | nil` (or `freeA <: 'T & C`) never
        // constrains 'T, because the FreeType path intercepts before structural dispatch.
        // Members may be GenericTypes that map to FreeTypes via genericSubstitutions.
        if FFlag::LuauPropagateFreeTypesIntoUnionAndIntersectionBounds.get() {
            let super_union = unsafe { get_type_id::<UnionType>(super_ty) };
            if !super_union.is_null() {
                let members: alloc::vec::Vec<TypeId> = unsafe { (*super_union).options.clone() };
                self.propagate_to_free_members(&members, sub_ty);
                return self.do_default_unify_free(sub_ty, super_ty);
            }

            let super_intersection = unsafe { get_type_id::<IntersectionType>(super_ty) };
            if !super_intersection.is_null() {
                let members: alloc::vec::Vec<TypeId> =
                    unsafe { (*super_intersection).parts.clone() };
                self.propagate_to_free_members(&members, sub_ty);
                return self.do_default_unify_free(sub_ty, super_ty);
            }
        }

        let super_function = unsafe { get_type_id::<FunctionType>(super_ty) };
        if super_function.is_null() {
            return self.do_default_unify_free(sub_ty, super_ty);
        }

        let (super_arg_head, super_arg_tail) =
            flatten_type_pack_id(unsafe { (*super_function).arg_types });
        if super_arg_tail.is_some() {
            return self.do_default_unify_free(sub_ty, super_ty);
        }

        let upper_bound_intersection = unsafe { get_type_id::<IntersectionType>(upper_bound) };
        if upper_bound_intersection.is_null() {
            return self.do_default_unify_free(sub_ty, super_ty);
        }

        let mut result = UnifyResult::Ok;
        let mut found_one = false;

        let parts: alloc::vec::Vec<TypeId> = unsafe { (*upper_bound_intersection).parts.clone() };
        for part in parts {
            let ft = unsafe { get_type_id::<FunctionType>(follow_type_id(part)) };
            if ft.is_null() {
                continue;
            }

            let (sub_arg_head, sub_arg_tail) = flatten_type_pack_id(unsafe { (*ft).arg_types });

            if sub_arg_tail.is_none() && sub_arg_head.len() == super_arg_head.len() {
                found_one = true;
                result &= self.unify_type_id_type_id(part, super_ty);
            }
        }

        if found_one {
            result
        } else {
            self.do_default_unify_free(sub_ty, super_ty)
        }
    }

    /// C++ `doDefault` lambda from `unifyFreeWithType`.
    fn do_default_unify_free(&mut self, sub_ty: TypeId, super_ty: TypeId) -> UnifyResult {
        let sub_free = unsafe { get_mutable_type_id::<FreeType>(sub_ty) };
        let new_super_ty = self.instantiate_with_bound_types(super_ty);
        let new_upper = self.mk_intersection(unsafe { (*sub_free).upper_bound }, new_super_ty);
        unsafe {
            (*sub_free).upper_bound = new_upper;
        }
        self.expanded_free_types
            .get_or_insert(sub_ty)
            .push(new_super_ty);
        UnifyResult::Ok
    }

    /// C++ `propagateToFreeMembers` lambda from `unifyFreeWithType`.
    fn propagate_to_free_members(&mut self, member_range: &[TypeId], sub_ty: TypeId) {
        for &member in member_range {
            let mut m = unsafe { follow_type_id(member) };
            if let Some(subst) = self.generic_substitutions.find(&m) {
                m = unsafe { follow_type_id(*subst) };
            }
            let member_free = unsafe { get_mutable_type_id::<FreeType>(m) };
            if !member_free.is_null() {
                let instantiated = self.instantiate_with_bound_types(sub_ty);
                let new_lower = self.mk_union(unsafe { (*member_free).lower_bound }, instantiated);
                unsafe {
                    (*member_free).lower_bound = new_lower;
                }
            }
        }
    }
}
