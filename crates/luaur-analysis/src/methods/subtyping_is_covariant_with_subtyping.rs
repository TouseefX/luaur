use crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy;
use crate::enums::table_state::TableState;
use crate::enums::type_field::TypeField;
use crate::functions::assert_reasoning_valid_subtyping::assert_reasoning_valid;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_2::get2;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::subsumes_scope::subsumes;
use crate::methods::subtyping_bind_generic::dense_hash_map_find_no_default;
use crate::records::any_type::AnyType;
use crate::records::blocked_type::BlockedType;
use crate::records::error_type::ErrorType;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::{PrimitiveType, Type as PrimType};
use crate::records::scope::Scope;
use crate::records::singleton_type::SingletonType;
use crate::records::subtype_constraint::SubtypeConstraint;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::component::Component;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_id::TypeId;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_ty: TypeId,
        super_ty: TypeId,
        scope: *mut Scope,
    ) -> SubtypingResult {
        // NonExceptionalRecursionLimiter nerl(&normalizer->sharedState->counters.recursionCount);
        let recursion_count_ptr =
            unsafe { &mut (*(*self.normalizer).shared_state).counters.recursion_count as *mut i32 };
        let mut nerl =
            crate::records::non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter {
                base: unsafe { core::mem::zeroed() },
                native_stack_guard: unsafe { core::mem::zeroed() },
            };
        nerl.non_exceptional_recursion_limiter_non_exceptional_recursion_limiter(
            recursion_count_ptr as *mut core::ffi::c_int,
        );
        if !nerl.is_ok(luaur_common::DFInt::LuauSubtypingRecursionLimit.get() as core::ffi::c_int) {
            return SubtypingResult {
                is_subtype: false,
                normalization_too_complex: true,
                ..Default::default()
            };
        }
        let _nerl = nerl;

        env.iteration_count += 1;
        let iteration_limit = luaur_common::FInt::LuauSubtypingIterationLimit.get() as i32;
        if iteration_limit > 0 && env.iteration_count >= iteration_limit {
            return SubtypingResult {
                is_subtype: false,
                normalization_too_complex: true,
                ..Default::default()
            };
        }

        let mut sub_ty = unsafe { follow_type_id(sub_ty) };
        let mut super_ty = unsafe { follow_type_id(super_ty) };

        if let Some(sub_it) = env.try_find_substitution(sub_ty) {
            if !sub_it.is_null() {
                sub_ty = sub_it;
            }
        }

        if let Some(super_it) = env.try_find_substitution(super_ty) {
            if !super_it.is_null() {
                // NOTE: This faithfully mirrors the C++, which assigns the
                // super substitution back into `subTy` (apparent upstream typo).
                sub_ty = super_it;
            }
        }

        if let Some(cached_result) = self.result_cache.find(&(sub_ty, super_ty)) {
            return cached_result.clone();
        }

        if let Some(cached_result) = env.try_find_subtyping_result((sub_ty, super_ty)) {
            return cached_result.clone();
        }

        // TODO: Do we care about returning a proof that this is error-suppressing?
        if sub_ty == super_ty {
            return SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
        }

        let type_pair = (sub_ty, super_ty);
        // seenTypes.insert(typePair) — Luau::Set semantics over a DenseHashMap<_, bool>.
        let fresh = {
            let entry = self.seen_types.get_or_insert(type_pair);
            let fresh = !*entry;
            if fresh {
                *entry = true;
            }
            fresh
        };
        if !fresh {
            // We've encountered a cycle; conservatively assume subtype and refuse to
            // cache anything that touches the cycle.
            let mut res = SubtypingResult::default();
            res.is_subtype = true;
            res.is_cacheable = false;

            *env.seen_set_cache.get_or_insert(type_pair) = res.clone();

            return res;
        }

        // ScopedSeenSet ssp{seenTypes, typePair}; — re-insert (no-op) and erase on
        // every exit below. With no `erase`, mirror `Luau::Set::erase` by clearing
        // the slot's bool before each return point.

        let mut result = SubtypingResult::default();

        let pair_ff = get2::<FreeType, FreeType, _>(sub_ty, super_ty);
        if !pair_ff.first.is_null() {
            // Any two free types are potentially subtypes of one another because
            // both of them could be narrowed to never.
            result = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
            result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
                sub_type: sub_ty,
                super_type: super_ty,
            }));
        } else if let Some(super_free) = unsafe { get_type_id::<FreeType>(super_ty).as_ref() } {
            // FIXME CLI-185582.
            result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env,
                sub_ty,
                super_free.upper_bound,
                scope,
            );

            if result.is_subtype {
                result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
                    sub_type: sub_ty,
                    super_type: super_ty,
                }));
            }
        } else if let Some(sub_free) = unsafe { get_type_id::<FreeType>(sub_ty).as_ref() } {
            // FIXME CLI-185582.
            if self
                .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                    env,
                    sub_free.lower_bound,
                    super_ty,
                    scope,
                )
                .is_subtype
            {
                result = SubtypingResult {
                    is_subtype: true,
                    ..Default::default()
                };
                result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
                    sub_type: sub_ty,
                    super_type: super_ty,
                }));
            } else {
                result = SubtypingResult {
                    is_subtype: false,
                    ..Default::default()
                };
            }
        } else if unsafe {
            !get_type_id::<BlockedType>(sub_ty).is_null()
                || !get_type_id::<BlockedType>(super_ty).is_null()
        } {
            result = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
            result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
                sub_type: sub_ty,
                super_type: super_ty,
            }));
        }
        // TODO: These branches are entirely incorrect (per upstream).
        else if let Some(sub_generic) = unsafe { get_type_id::<GenericType>(sub_ty).as_ref() }
            .filter(|g| subsumes(g.scope, scope))
        {
            let _ = sub_generic;
            self.seen_set_erase(type_pair);
            return self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env,
                unsafe { (*self.builtin_types).neverType },
                super_ty,
                scope,
            );
        } else if let Some(super_generic) = unsafe { get_type_id::<GenericType>(super_ty).as_ref() }
            .filter(|g| subsumes(g.scope, scope))
        {
            let _ = super_generic;
            self.seen_set_erase(type_pair);
            return self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env,
                sub_ty,
                unsafe { (*self.builtin_types).unknownType },
                scope,
            );
        } else if unsafe { !get_type_id::<AnyType>(super_ty).is_null() } {
            result = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
        }
        // any = err | unknown.
        else if unsafe {
            !get_type_id::<AnyType>(sub_ty).is_null()
                && !get_type_id::<UnknownType>(super_ty).is_null()
        } {
            result = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
        } else if unsafe { !get_type_id::<AnyType>(sub_ty).is_null() } {
            // any = unknown | error, so we rewrite this to match.
            result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env,
                unsafe { (*self.builtin_types).unknownType },
                super_ty,
                scope,
            );
            let err_part = self
                .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                    env,
                    unsafe { (*self.builtin_types).errorType },
                    super_ty,
                    scope,
                );
            result.and_also(err_part, SubtypingSuppressionPolicy::Any);
            result.is_error_suppressing = true;
        } else if unsafe {
            !get_type_id::<UnknownType>(super_ty).is_null()
                && get_type_id::<UnionType>(sub_ty).is_null()
                && get_type_id::<IntersectionType>(sub_ty).is_null()
        } {
            let error_suppressing = unsafe { !get_type_id::<ErrorType>(sub_ty).is_null() };
            result.is_subtype = !error_suppressing;
            result.is_error_suppressing = error_suppressing;
        } else if unsafe { !get_type_id::<NeverType>(sub_ty).is_null() } {
            result = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
        } else if unsafe { !get_type_id::<ErrorType>(super_ty).is_null() } {
            result = SubtypingResult {
                is_subtype: false,
                ..Default::default()
            };
        } else if unsafe { !get_type_id::<ErrorType>(sub_ty).is_null() } {
            result = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
            result.is_error_suppressing = true;
        } else if unsafe { !get_type_id::<TypeFunctionInstanceType>(sub_ty).is_null() } {
            let mut sub_type_function_instance =
                unsafe { get_type_id::<TypeFunctionInstanceType>(sub_ty) };
            let mut mapped_generics_applied = false;
            if let Some(subst_sub_ty) =
                env.apply_mapped_generics(self.builtin_types, self.arena, sub_ty, self.ice_reporter)
            {
                mapped_generics_applied = subst_sub_ty != sub_ty;
                sub_type_function_instance =
                    unsafe { get_type_id::<TypeFunctionInstanceType>(subst_sub_ty) };
            }

            result = self
                .is_covariant_with_subtyping_environment_type_function_instance_type_type_id_not_null_scope(
                    env,
                    unsafe { &*sub_type_function_instance },
                    super_ty,
                    scope,
                );
            result.is_cacheable = !mapped_generics_applied;
        } else if unsafe { !get_type_id::<TypeFunctionInstanceType>(super_ty).is_null() } {
            let mut super_type_function_instance =
                unsafe { get_type_id::<TypeFunctionInstanceType>(super_ty) };
            let mut mapped_generics_applied = false;
            if let Some(subst_super_ty) = env.apply_mapped_generics(
                self.builtin_types,
                self.arena,
                super_ty,
                self.ice_reporter,
            ) {
                mapped_generics_applied = subst_super_ty != super_ty;
                super_type_function_instance =
                    unsafe { get_type_id::<TypeFunctionInstanceType>(subst_super_ty) };
            }

            result = self
                .is_covariant_with_subtyping_environment_type_id_type_function_instance_type_not_null_scope(
                    env,
                    sub_ty,
                    unsafe { &*super_type_function_instance },
                    scope,
                );
            result.is_cacheable = !mapped_generics_applied;
        } else if unsafe {
            !get_type_id::<GenericType>(sub_ty).is_null()
                || !get_type_id::<GenericType>(super_ty).is_null()
        } {
            let sub_has_bounds = dense_hash_map_find_no_default(&env.mapped_generics, &sub_ty)
                .map_or(false, |b| !b.is_empty());
            let super_has_bounds = dense_hash_map_find_no_default(&env.mapped_generics, &super_ty)
                .map_or(false, |b| !b.is_empty());
            if sub_has_bounds {
                let ok = self.bind_generic(env, sub_ty, super_ty);
                result.is_subtype = ok;
                result.is_cacheable = false;
            } else if super_has_bounds {
                let ok = self.bind_generic(env, sub_ty, super_ty);
                result.is_subtype = ok;
                result.is_cacheable = false;
            }
        } else if let Some(sub_union) = unsafe { get_type_id::<UnionType>(sub_ty).as_ref() } {
            result = self
                .is_covariant_with_subtyping_environment_union_type_type_id_not_null_scope(
                    env, sub_union, super_ty, scope,
                );
        } else if let Some(super_union) = unsafe { get_type_id::<UnionType>(super_ty).as_ref() } {
            result = self
                .is_covariant_with_subtyping_environment_type_id_union_type_not_null_scope(
                    env,
                    sub_ty,
                    super_union,
                    scope,
                );
            if !result.is_subtype && !result.normalization_too_complex {
                result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
            }
        } else if let Some(super_intersection) =
            unsafe { get_type_id::<IntersectionType>(super_ty).as_ref() }
        {
            result = self
                .is_covariant_with_subtyping_environment_type_id_intersection_type_not_null_scope(
                    env,
                    sub_ty,
                    super_intersection,
                    scope,
                );
        } else if let Some(sub_intersection) =
            unsafe { get_type_id::<IntersectionType>(sub_ty).as_ref() }
        {
            result = self
                .is_covariant_with_subtyping_environment_intersection_type_type_id_not_null_scope(
                    env,
                    sub_intersection,
                    super_ty,
                    scope,
                );
            if !result.is_subtype && !result.normalization_too_complex {
                result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
            }
        } else if {
            let p = get2::<NegationType, NegationType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<NegationType, NegationType, _>(sub_ty, super_ty);
            // We use `isContravariantWith` here in order to make sure that the
            // type paths still look coherent.
            result = self
                .is_contravariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
                    env,
                    unsafe { (*p.first).ty },
                    unsafe { (*p.second).ty },
                    scope,
                );
            result.with_both_component(Component::TypeField(TypeField::Negated));
        } else if let Some(sub_negation) = unsafe { get_type_id::<NegationType>(sub_ty).as_ref() } {
            result = self
                .is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
                    env,
                    sub_negation,
                    super_ty,
                    scope,
                );
            if !result.is_subtype && !result.normalization_too_complex {
                result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
            }
        } else if let Some(super_negation) =
            unsafe { get_type_id::<NegationType>(super_ty).as_ref() }
        {
            result = self
                .is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
                    env,
                    sub_ty,
                    super_negation,
                    scope,
                );
            if !result.is_subtype && !result.normalization_too_complex {
                result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
            }
        } else if {
            let p = get2::<PrimitiveType, PrimitiveType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<PrimitiveType, PrimitiveType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_primitive_type_primitive_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
        } else if {
            let p = get2::<SingletonType, PrimitiveType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<SingletonType, PrimitiveType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_singleton_type_primitive_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
        } else if {
            let p = get2::<SingletonType, SingletonType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<SingletonType, SingletonType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_singleton_type_singleton_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
        } else if {
            let p = get2::<FunctionType, PrimitiveType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<FunctionType, PrimitiveType, _>(sub_ty, super_ty);
            let _sub_function = p.first;
            let super_primitive = p.second;
            result.is_subtype = unsafe { (*super_primitive).r#type == PrimType::Function };
        } else if {
            let p = get2::<FunctionType, FunctionType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<FunctionType, FunctionType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_function_type_function_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
        } else if {
            let p = get2::<TableType, TableType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<TableType, TableType, _>(sub_ty, super_ty);
            let force_covariant_test =
                !self.unique_types.is_null() && unsafe { (*self.unique_types).contains(&sub_ty) };
            result = if luaur_common::FFlag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
                self.is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    force_covariant_test,
                    scope,
                )
            } else {
                self.is_covariant_with_deprecated(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    force_covariant_test,
                    scope,
                )
            };
            if result.is_subtype
                && unsafe { (*p.first).indexer.is_none() }
                && unsafe { (*p.second).indexer.is_some() }
                && unsafe { (*p.first).state != TableState::Sealed }
            {
                // FIXME CLI-182960.
                result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
                    sub_type: sub_ty,
                    super_type: super_ty,
                }));
            }
        } else if {
            let p = get2::<MetatableType, MetatableType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<MetatableType, MetatableType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_metatable_type_metatable_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
        } else if {
            let p = get2::<MetatableType, TableType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<MetatableType, TableType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_metatable_type_table_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
        } else if luaur_common::FFlag::LuauTableFreezeCheckIsSubtype.get() && {
            let p = get2::<MetatableType, PrimitiveType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<MetatableType, PrimitiveType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_metatable_type_primitive_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
        } else if {
            let p = get2::<ExternType, ExternType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<ExternType, ExternType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_extern_type_extern_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
        } else if {
            let p = get2::<ExternType, TableType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<ExternType, TableType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_type_id_extern_type_type_id_table_type_not_null_scope(
                    env,
                    sub_ty,
                    unsafe { &*p.first },
                    super_ty,
                    unsafe { &*p.second },
                    scope,
                );
        } else if {
            let p = get2::<TableType, PrimitiveType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<TableType, PrimitiveType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_table_type_primitive_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
        } else if {
            let p = get2::<PrimitiveType, TableType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<PrimitiveType, TableType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_primitive_type_table_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
        } else if {
            let p = get2::<SingletonType, TableType, _>(sub_ty, super_ty);
            !p.first.is_null()
        } {
            let p = get2::<SingletonType, TableType, _>(sub_ty, super_ty);
            result = self
                .is_covariant_with_subtyping_environment_singleton_type_table_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
        }

        assert_reasoning_valid(sub_ty, super_ty, &result, self.builtin_types, self.arena);

        // ScopedSeenSet destructor — erase the pair before returning.
        self.seen_set_erase(type_pair);

        self.cache(env, result, sub_ty, super_ty)
    }

    /// Mirror `Luau::Set::erase` over `seen_types` (a `DenseHashMap<_, bool>`): set
    /// the slot's value to `false` rather than removing the key.
    #[inline]
    fn seen_set_erase(&mut self, type_pair: (TypeId, TypeId)) {
        let entry = self.seen_types.get_or_insert(type_pair);
        if *entry {
            *entry = false;
        }
    }
}
