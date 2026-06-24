//! Source: `Analysis/src/Unifier2.cpp:669-807` — `Unifier2::unify_(TypePackId, TypePackId)`,
//! the type-pack core of new-solver unification.

use crate::enums::occurs_check_result::OccursCheckResult;
use crate::enums::unify_result::UnifyResult;
use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::functions::emplace_type_pack::emplace_type_pack;
use crate::functions::extend_type_pack::extend_type_pack;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::is_irresolvable_unifier_2_alt_b::is_irresolvable;
use crate::functions::occurs_check_type_utils_alt_b::occurs_check_type_pack_id_type_pack_id;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter;
use crate::records::pack_subtype_constraint::PackSubtypeConstraint;
use crate::records::replacer::Replacer;
use crate::records::type_pack::TypePack;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use core::ptr::NonNull;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::{FFlag, FInt};

impl Unifier2 {
    pub fn unify_type_pack_id_type_pack_id(
        &mut self,
        mut sub_tp: TypePackId,
        mut super_tp: TypePackId,
    ) -> UnifyResult {
        if FInt::LuauTypeInferIterationLimit.get() > 0
            && self.iteration_count >= FInt::LuauTypeInferIterationLimit.get() as i32
        {
            return UnifyResult::TooComplex;
        }

        self.iteration_count += 1;

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

        sub_tp = unsafe { follow_type_pack_id(sub_tp) };
        super_tp = unsafe { follow_type_pack_id(super_tp) };

        if self.seen_type_pack_pairings.contains(&(sub_tp, super_tp)) {
            return UnifyResult::Ok;
        }
        self.seen_type_pack_pairings.insert((sub_tp, super_tp));

        if sub_tp == super_tp {
            return UnifyResult::Ok;
        }

        // FIXME: CLI-188000: If we are _directly_ given a free type, we must
        // eagerly emplace it. Otherwise, later, we may generalize the underlying
        // free types incorrectly.
        if !unsafe { get_type_pack_id::<FreeTypePack>(sub_tp) }.is_null() {
            return self.emplace_free_type_pack(sub_tp, super_tp);
        }

        if !unsafe { get_type_pack_id::<FreeTypePack>(super_tp) }.is_null() {
            return self.emplace_free_type_pack(super_tp, sub_tp);
        }

        let sub_len = flatten_type_pack_id(sub_tp).0.len();
        let super_len = flatten_type_pack_id(super_tp).0.len();
        let max_length = core::cmp::max(sub_len, super_len);

        let arena = self.arena.as_ptr();
        let builtin_types_ptr = self.builtin_types.as_ptr();

        let sub_extended = extend_type_pack(
            unsafe { &mut *arena },
            builtin_types_ptr,
            sub_tp,
            max_length,
            alloc::vec::Vec::new(),
        );
        let super_extended = extend_type_pack(
            unsafe { &mut *arena },
            builtin_types_ptr,
            super_tp,
            max_length,
            alloc::vec::Vec::new(),
        );

        let sub_types = sub_extended.head;
        let sub_tail = sub_extended.tail;
        let super_types = super_extended.head;
        let super_tail = super_extended.tail;

        let limit = core::cmp::min(sub_types.len(), super_types.len());
        for i in 0..limit {
            self.unify_type_id_type_id(sub_types[i], super_types[i]);
        }

        // At this point it should be the case that either:
        // - `subTypes` now has all of its types unified, and we are down to its tail
        // - `superTypes` now has all of its types unified, and we are down to its tail

        if sub_tail.is_none() && super_tail.is_none() {
            // If both types are missing a tail, we've done all we can.
            return UnifyResult::Ok;
        }

        // It should be the case that exclusively one of these packs can be reduced
        // to their tail for the rest of the function.
        if limit < sub_types.len() {
            LUAU_ASSERT!(limit == super_types.len());
            // If we have extra subtypes left over, construct a new type pack
            let new_sub_head: alloc::vec::Vec<TypeId> = sub_types[super_types.len()..].to_vec();
            sub_tp = unsafe { &mut *arena }.add_type_pack_t(TypePack {
                head: new_sub_head,
                tail: sub_tail,
            });
            super_tp = self.maybe_replace_tail(super_tail);
        } else if limit < super_types.len() {
            LUAU_ASSERT!(limit == sub_types.len() && limit < super_types.len());
            // If we have extra subtypes left over, construct a new type pack
            let new_super_head: alloc::vec::Vec<TypeId> = super_types[sub_types.len()..].to_vec();
            super_tp = unsafe { &mut *arena }.add_type_pack_t(TypePack {
                head: new_super_head,
                tail: super_tail,
            });
            sub_tp = self.maybe_replace_tail(sub_tail);
        } else {
            sub_tp = self.maybe_replace_tail(sub_tail);
            super_tp = self.maybe_replace_tail(super_tail);
        }

        if is_irresolvable(sub_tp) || is_irresolvable(super_tp) {
            if !self.uninhabited_type_functions.is_null()
                && unsafe {
                    (*self.uninhabited_type_functions)
                        .contains(&(sub_tp as *const core::ffi::c_void))
                        || (*self.uninhabited_type_functions)
                            .contains(&(super_tp as *const core::ffi::c_void))
                }
            {
                return UnifyResult::Ok;
            }

            self.incomplete_subtypes
                .push(ConstraintV::PackSubtype(PackSubtypeConstraint {
                    sub_pack: sub_tp,
                    super_pack: super_tp,
                    returns: false,
                }));
            return UnifyResult::Ok;
        }

        // ... after doing all of our replacements, we may also need to check for
        // free types again.

        if !unsafe { get_type_pack_id::<FreeTypePack>(sub_tp) }.is_null() {
            return self.emplace_free_type_pack(sub_tp, super_tp);
        }

        if !unsafe { get_type_pack_id::<FreeTypePack>(super_tp) }.is_null() {
            return self.emplace_free_type_pack(super_tp, sub_tp);
        }

        UnifyResult::Ok
    }

    /// C++ `emplaceFreeTypePack` lambda from `unify_(TypePackId, TypePackId)`.
    fn emplace_free_type_pack(&mut self, target: TypePackId, bound_to: TypePackId) -> UnifyResult {
        LUAU_ASSERT!(!unsafe { get_type_pack_id::<FreeTypePack>(target) }.is_null());

        let bound_to = self.instantiate_with_bound_types_pack(bound_to);

        let error_pack = unsafe { (*self.builtin_types.as_ptr()).errorTypePack };

        if FFlag::LuauOccursCheckForAllBindings.get() {
            if occurs_check_type_pack_id_type_pack_id(target, bound_to) == OccursCheckResult::Fail {
                emplace_type_pack(
                    unsafe { as_mutable_type_pack_id(target) },
                    TypePackVariant::Bound(error_pack),
                );
                return UnifyResult::OccursCheckFailed;
            }
        } else {
            let mut seen: DenseHashSet<TypePackId> = DenseHashSet::new(core::ptr::null());
            if OccursCheckResult::Fail == self.occurs_check_deprecated(&mut seen, target, bound_to)
            {
                emplace_type_pack(
                    unsafe { as_mutable_type_pack_id(target) },
                    TypePackVariant::Bound(error_pack),
                );
                return UnifyResult::OccursCheckFailed;
            }
        }

        emplace_type_pack(
            unsafe { as_mutable_type_pack_id(target) },
            TypePackVariant::Bound(bound_to),
        );
        UnifyResult::Ok
    }

    /// C++ `maybeReplaceTail` lambda from `unify_(TypePackId, TypePackId)`.
    fn maybe_replace_tail(&self, maybe_tp: Option<TypePackId>) -> TypePackId {
        let maybe_tp = match maybe_tp {
            None => return unsafe { (*self.builtin_types.as_ptr()).emptyTypePack },
            Some(tp) => tp,
        };

        let tp = unsafe { follow_type_pack_id(maybe_tp) };
        if let Some(replacement) = self.generic_pack_substitutions.find(&tp) {
            return unsafe { follow_type_pack_id(*replacement) };
        }
        tp
    }

    /// `instantiateWithBoundTypes` for type packs (`Unifier2.cpp:307-314`, TypePackId
    /// instantiation of the `template<typename TID>` overload).
    fn instantiate_with_bound_types_pack(&mut self, tp: TypePackId) -> TypePackId {
        let mut r = Replacer::replacer(
            self.arena.as_ptr(),
            NonNull::from(&mut self.generic_substitutions).as_ptr(),
            NonNull::from(&mut self.generic_pack_substitutions).as_ptr(),
        );
        if let Some(new_tp) = r.substitute_type_pack_id(tp) {
            return new_tp;
        }
        tp
    }
}
