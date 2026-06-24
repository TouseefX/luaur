//! Source: `Analysis/src/Normalize.cpp:1796-1967` (hand-ported)
use crate::enums::normalization_result::NormalizationResult;
use crate::functions::assert_invariant::assert_invariant;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_singleton_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_cacheable_normalize_alt_c::is_cacheable_type_id;
use crate::functions::tyvar_index::tyvar_index;
use crate::records::any_type::AnyType;
use crate::records::blocked_type::BlockedType;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::no_refine_type::NoRefineType;
use crate::records::normalized_extern_type::NormalizedExternType;
use crate::records::normalized_function_type::NormalizedFunctionType;
use crate::records::normalized_string_type::NormalizedStringType;
use crate::records::normalized_type::NormalizedType;
use crate::records::normalizer::Normalizer;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::primitive_type::{PrimitiveType, Type as PrimType};
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_ids::TypeIds;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::seen_table_prop_pairs::SeenTablePropPairs;
use crate::type_aliases::type_id::TypeId;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::FFlag;

/// C++ `Set<TypeId>::erase(key)`. The Rust skeleton maps the seen-set parameter
/// to `DenseHashSet`, which (faithful to `Luau::DenseHashSet`) cannot erase a
/// single slot, whereas C++ here uses `Luau::Set` which can. We reproduce the
/// single-element removal by rebuilding the set without `key`; `clear` preserves
/// the empty-key sentinel, so the rebuilt set stays valid.
pub(crate) fn erase_seen(seen: &mut DenseHashSet<TypeId>, key: TypeId) {
    let kept: Vec<TypeId> = seen.iter().copied().filter(|&k| k != key).collect();
    seen.clear();
    for k in kept {
        seen.insert(k);
    }
}

/// RAII guard mirroring C++ `RecursionCounter _rc(&sharedState->counters.recursionCount)`:
/// increments the shared recursion counter on construction, decrements on drop.
struct RcGuard {
    count: *mut i32,
}

impl RcGuard {
    fn new(count: *mut i32) -> Self {
        unsafe {
            *count += 1;
        }
        RcGuard { count }
    }
}

impl Drop for RcGuard {
    fn drop(&mut self) {
        unsafe {
            *self.count -= 1;
        }
    }
}

fn fresh_normalized_type(
    builtin_types: *mut crate::records::builtin_types::BuiltinTypes,
) -> NormalizedType {
    let never_type = unsafe { (*builtin_types).neverType };
    NormalizedType {
        builtin_types,
        tops: never_type,
        booleans: never_type,
        extern_types: NormalizedExternType {
            extern_types: BTreeMap::new(),
            shape_extensions: TypeIds::type_ids(),
            ordering: Vec::new(),
        },
        errors: never_type,
        nils: never_type,
        numbers: never_type,
        integers: never_type,
        strings: NormalizedStringType::never,
        threads: never_type,
        buffers: never_type,
        tables: TypeIds::type_ids(),
        functions: NormalizedFunctionType {
            is_top: false,
            parts: TypeIds::type_ids(),
        },
        tyvars: BTreeMap::new(),
        is_cacheable: true,
    }
}

impl Normalizer {
    // See above for an explanation of `ignoreSmallerTyvars`.
    pub fn union_normal_with_ty(
        &mut self,
        here: &mut NormalizedType,
        there: TypeId,
        seen_table_prop_pairs: &mut SeenTablePropPairs,
        seen_set_types: &mut DenseHashSet<TypeId>,
        ignore_smaller_tyvars: i32,
    ) -> NormalizationResult {
        let _rc = RcGuard::new(unsafe { &mut (*self.shared_state).counters.recursion_count });
        if !self.within_resource_limits() {
            return NormalizationResult::HitLimits;
        }

        self.consume_fuel();

        let there = unsafe { follow_type_id(there) };

        if !unsafe { get_type_id::<AnyType>(there).is_null() }
            || !unsafe { get_type_id::<UnknownType>(there).is_null() }
        {
            let mut tops = self.union_of_tops(here.tops, there);
            if !unsafe { get_type_id::<UnknownType>(tops).is_null() }
                && !unsafe { get_type_id::<ErrorType>(here.errors).is_null() }
            {
                tops = unsafe { (*self.builtin_types).anyType };
            }
            self.clear_normal(here);
            here.tops = tops;
            return NormalizationResult::True;
        } else if !unsafe { get_type_id::<NeverType>(there).is_null() }
            || !unsafe { get_type_id::<AnyType>(here.tops).is_null() }
        {
            return NormalizationResult::True;
        } else if !unsafe { get_type_id::<ErrorType>(there).is_null() }
            && !unsafe { get_type_id::<UnknownType>(here.tops).is_null() }
        {
            here.tops = unsafe { (*self.builtin_types).anyType };
            return NormalizationResult::True;
        } else if !unsafe { get_type_id::<crate::records::union_type::UnionType>(there).is_null() }
        {
            if seen_set_types.contains(&there) {
                return NormalizationResult::True;
            }
            seen_set_types.insert(there);

            let options = unsafe {
                (*get_type_id::<crate::records::union_type::UnionType>(there))
                    .options
                    .clone()
            };
            for opt in options {
                let res =
                    self.union_normal_with_ty(here, opt, seen_table_prop_pairs, seen_set_types, -1);
                if res != NormalizationResult::True {
                    erase_seen(seen_set_types, there);
                    return res;
                }
            }

            erase_seen(seen_set_types, there);
            return NormalizationResult::True;
        } else if !unsafe { get_type_id::<IntersectionType>(there).is_null() } {
            if seen_set_types.contains(&there) {
                return NormalizationResult::True;
            }
            seen_set_types.insert(there);

            let mut norm = fresh_normalized_type(self.builtin_types);
            norm.tops = unsafe { (*self.builtin_types).unknownType };
            let parts = unsafe { (*get_type_id::<IntersectionType>(there)).parts.clone() };
            for part in parts {
                let res = self.intersect_normal_with_ty(
                    &mut norm,
                    part,
                    seen_table_prop_pairs,
                    seen_set_types,
                );
                if res != NormalizationResult::True {
                    erase_seen(seen_set_types, there);
                    return res;
                }
            }

            erase_seen(seen_set_types, there);

            return self.union_normals(here, &norm, -1);
        } else if !unsafe { get_type_id::<UnknownType>(here.tops).is_null() } {
            return NormalizationResult::True;
        } else if !unsafe { get_type_id::<GenericType>(there).is_null() }
            || !unsafe { get_type_id::<FreeType>(there).is_null() }
            || !unsafe { get_type_id::<BlockedType>(there).is_null() }
            || !unsafe { get_type_id::<PendingExpansionType>(there).is_null() }
            || !unsafe { get_type_id::<TypeFunctionInstanceType>(there).is_null() }
        {
            if tyvar_index(there) <= ignore_smaller_tyvars {
                return NormalizationResult::True;
            }
            let mut inter = fresh_normalized_type(self.builtin_types);
            inter.tops = unsafe { (*self.builtin_types).unknownType };
            here.tyvars.insert(there, Box::new(inter));

            if !is_cacheable_type_id(there) {
                here.is_cacheable = false;
            }
        } else if !unsafe { get_type_id::<FunctionType>(there).is_null() } {
            self.union_functions_with_function(&mut here.functions, there);
        } else if !unsafe { get_type_id::<TableType>(there).is_null() }
            || !unsafe { get_type_id::<MetatableType>(there).is_null() }
        {
            self.union_tables_with_table(&mut here.tables, there);
        } else if !unsafe { get_type_id::<ExternType>(there).is_null() } {
            self.union_extern_types_with_extern_type_normalized_extern_type_type_id(
                &mut here.extern_types,
                there,
            );
        } else if !unsafe { get_type_id::<ErrorType>(there).is_null() } {
            here.errors = there;
        } else if !unsafe { get_type_id::<PrimitiveType>(there).is_null() } {
            let ptv = unsafe { &*get_type_id::<PrimitiveType>(there) };
            match ptv.r#type {
                PrimType::Boolean => here.booleans = there,
                PrimType::NilType => here.nils = there,
                PrimType::Number => here.numbers = there,
                PrimType::Integer if FFlag::LuauIntegerType2.get() => here.integers = there,
                PrimType::String => {
                    crate::methods::normalized_string_type_reset_to_string::normalized_string_type_reset_to_string(
                        &mut here.strings,
                    )
                }
                PrimType::Thread => here.threads = there,
                PrimType::Buffer => here.buffers = there,
                PrimType::Function => here.functions.reset_to_top(),
                PrimType::Table => {
                    here.tables.clear();
                    here.tables.insert_type_id(there);
                }
                _ => LUAU_ASSERT!(false),
            }
        } else if !unsafe { get_type_id::<SingletonType>(there).is_null() } {
            let stv = unsafe { get_type_id::<SingletonType>(there) };
            if !get_singleton_type::<BooleanSingleton>(stv).is_null() {
                here.booleans = self.union_of_bools(here.booleans, there);
            } else if !get_singleton_type::<StringSingleton>(stv).is_null() {
                let sstv = unsafe { &*get_singleton_type::<StringSingleton>(stv) };
                if here.strings.isCofinite {
                    if here.strings.singletons.contains_key(&sstv.value) {
                        here.strings.singletons.remove(&sstv.value);
                    }
                } else {
                    here.strings.singletons.insert(sstv.value.clone(), there);
                }
            } else {
                LUAU_ASSERT!(false);
            }
        } else if !unsafe { get_type_id::<NegationType>(there).is_null() } {
            let ntv_ty = unsafe { (*get_type_id::<NegationType>(there)).ty };

            let there_normal = self.normalize(ntv_ty);
            let tn = self.negate_normal(&there_normal);

            let mut tn = match tn {
                Some(t) => t,
                None => return NormalizationResult::False,
            };

            let res = self.union_normals(here, &mut tn, -1);
            if res != NormalizationResult::True {
                return res;
            }
        } else if !unsafe { get_type_id::<PendingExpansionType>(there).is_null() }
            || !unsafe { get_type_id::<TypeFunctionInstanceType>(there).is_null() }
            || !unsafe { get_type_id::<NoRefineType>(there).is_null() }
        {
            // nothing
        } else {
            LUAU_ASSERT!(false);
        }

        let tyvar_keys: Vec<TypeId> = here.tyvars.keys().copied().collect();
        for tyvar in tyvar_keys {
            if let Some(mut intersect) = here.tyvars.remove(&tyvar) {
                let res = self.union_normal_with_ty(
                    &mut intersect,
                    there,
                    seen_table_prop_pairs,
                    seen_set_types,
                    tyvar_index(tyvar),
                );
                here.tyvars.insert(tyvar, intersect);
                if res != NormalizationResult::True {
                    return res;
                }
            }
        }

        assert_invariant(here);
        NormalizationResult::True
    }
}
