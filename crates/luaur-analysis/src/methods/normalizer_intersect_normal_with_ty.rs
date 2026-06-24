//! Source: `Analysis/src/Normalize.cpp:3319-3564` (hand-ported)
use crate::enums::normalization_result::NormalizationResult;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_singleton_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::should_early_exit::should_early_exit;
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
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::normalized_tyvars::NormalizedTyvars;
use crate::type_aliases::seen_table_prop_pairs::SeenTablePropPairs;
use crate::type_aliases::type_id::TypeId;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use core::mem;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::FFlag;

/// RAII guard mirroring C++ `RecursionCounter _rc(&sharedState->counters.recursionCount)`.
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
    pub fn intersect_normal_with_ty(
        &mut self,
        here: &mut NormalizedType,
        there: TypeId,
        seen_table_prop_pairs: &mut SeenTablePropPairs,
        seen_set_types: &mut DenseHashSet<TypeId>,
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
            here.tops = self.intersection_of_tops(here.tops, there);
            return NormalizationResult::True;
        } else if unsafe { get_type_id::<NeverType>(here.tops).is_null() } {
            self.clear_normal(here);
            return self.union_normal_with_ty(
                here,
                there,
                seen_table_prop_pairs,
                seen_set_types,
                -1,
            );
        } else if !unsafe { get_type_id::<UnionType>(there).is_null() } {
            let mut norm = fresh_normalized_type(self.builtin_types);
            let options = unsafe { (*get_type_id::<UnionType>(there)).options.clone() };
            for opt in options {
                let res = self.union_normal_with_ty(
                    &mut norm,
                    opt,
                    seen_table_prop_pairs,
                    seen_set_types,
                    -1,
                );
                if res != NormalizationResult::True {
                    return res;
                }
            }
            return self.intersect_normals(here, &norm, -1);
        } else if !unsafe { get_type_id::<IntersectionType>(there).is_null() } {
            let parts = unsafe { (*get_type_id::<IntersectionType>(there)).parts.clone() };
            for part in parts {
                let res = self.intersect_normal_with_ty(
                    here,
                    part,
                    seen_table_prop_pairs,
                    seen_set_types,
                );
                if res != NormalizationResult::True {
                    return res;
                }
            }
            return NormalizationResult::True;
        } else if !unsafe { get_type_id::<GenericType>(there).is_null() }
            || !unsafe { get_type_id::<FreeType>(there).is_null() }
            || !unsafe { get_type_id::<BlockedType>(there).is_null() }
            || !unsafe { get_type_id::<PendingExpansionType>(there).is_null() }
            || !unsafe { get_type_id::<TypeFunctionInstanceType>(there).is_null() }
        {
            let mut there_norm = fresh_normalized_type(self.builtin_types);
            let mut top_norm = fresh_normalized_type(self.builtin_types);
            top_norm.tops = unsafe { (*self.builtin_types).unknownType };
            there_norm.tyvars.insert(there, Box::new(top_norm));
            here.is_cacheable = false;
            return self.intersect_normals(here, &there_norm, -1);
        }

        let mut tyvars: NormalizedTyvars = mem::take(&mut here.tyvars);

        if !unsafe { get_type_id::<FunctionType>(there).is_null() } {
            let mut functions = mem::replace(
                &mut here.functions,
                NormalizedFunctionType {
                    is_top: false,
                    parts: TypeIds::type_ids(),
                },
            );
            self.clear_normal(here);
            self.intersect_functions_with_function(&mut functions, there);
            here.functions = functions;
        } else if !unsafe { get_type_id::<TableType>(there).is_null() }
            || !unsafe { get_type_id::<MetatableType>(there).is_null() }
        {
            if self.use_new_luau_solver() {
                let mut extern_types = mem::replace(
                    &mut here.extern_types,
                    NormalizedExternType {
                        extern_types: BTreeMap::new(),
                        shape_extensions: TypeIds::type_ids(),
                        ordering: Vec::new(),
                    },
                );
                let mut tables = mem::replace(&mut here.tables, TypeIds::type_ids());
                self.clear_normal(here);

                if FFlag::LuauExternTypesNormalizeWithShapes.get() {
                    if extern_types.is_never() {
                        self.intersect_tables_with_table(
                            &mut tables,
                            there,
                            seen_table_prop_pairs,
                            seen_set_types,
                        );
                    } else {
                        self.intersect_extern_types_with_shape(&mut extern_types, there);
                    }
                } else {
                    self.intersect_tables_with_table(
                        &mut tables,
                        there,
                        seen_table_prop_pairs,
                        seen_set_types,
                    );
                }

                here.tables = tables;
                here.extern_types = extern_types;
            } else {
                let mut tables = mem::replace(&mut here.tables, TypeIds::type_ids());
                self.clear_normal(here);
                self.intersect_tables_with_table(
                    &mut tables,
                    there,
                    seen_table_prop_pairs,
                    seen_set_types,
                );
                here.tables = tables;
            }
        } else if !unsafe { get_type_id::<ExternType>(there).is_null() } {
            let mut nct = mem::replace(
                &mut here.extern_types,
                NormalizedExternType {
                    extern_types: BTreeMap::new(),
                    shape_extensions: TypeIds::type_ids(),
                    ordering: Vec::new(),
                },
            );
            self.clear_normal(here);
            self.intersect_extern_types_with_extern_type(&mut nct, there);
            here.extern_types = nct;
        } else if !unsafe { get_type_id::<ErrorType>(there).is_null() } {
            let errors = here.errors;
            self.clear_normal(here);
            here.errors = if !unsafe { get_type_id::<ErrorType>(errors).is_null() } {
                errors
            } else {
                there
            };
        } else if !unsafe { get_type_id::<PrimitiveType>(there).is_null() } {
            let booleans = here.booleans;
            let nils = here.nils;
            let numbers = here.numbers;
            let integers = here.integers;
            let strings = mem::replace(&mut here.strings, NormalizedStringType::never);
            let functions = mem::replace(
                &mut here.functions,
                NormalizedFunctionType {
                    is_top: false,
                    parts: TypeIds::type_ids(),
                },
            );
            let threads = here.threads;
            let buffers = here.buffers;
            let tables = mem::replace(&mut here.tables, TypeIds::type_ids());

            self.clear_normal(here);

            let ptv = unsafe { &*get_type_id::<PrimitiveType>(there) };
            match ptv.r#type {
                PrimType::Boolean => here.booleans = booleans,
                PrimType::NilType => here.nils = nils,
                PrimType::Number => here.numbers = numbers,
                PrimType::Integer if FFlag::LuauIntegerType2.get() => here.integers = integers,
                PrimType::String => here.strings = strings,
                PrimType::Thread => here.threads = threads,
                PrimType::Buffer => here.buffers = buffers,
                PrimType::Function => here.functions = functions,
                PrimType::Table => here.tables = tables,
                _ => LUAU_ASSERT!(false),
            }
        } else if !unsafe { get_type_id::<SingletonType>(there).is_null() } {
            let booleans = here.booleans;
            let strings = mem::replace(&mut here.strings, NormalizedStringType::never);

            self.clear_normal(here);

            let stv = unsafe { get_type_id::<SingletonType>(there) };
            if !get_singleton_type::<BooleanSingleton>(stv).is_null() {
                here.booleans = self.intersection_of_bools(booleans, there);
            } else if !get_singleton_type::<StringSingleton>(stv).is_null() {
                let sstv = unsafe { &*get_singleton_type::<StringSingleton>(stv) };
                if strings.includes(&sstv.value) {
                    here.strings.singletons.insert(sstv.value.clone(), there);
                }
            } else {
                LUAU_ASSERT!(false);
            }
        } else if !unsafe { get_type_id::<NegationType>(there).is_null() } {
            let ntv_ty = unsafe { (*get_type_id::<NegationType>(there)).ty };
            let t = unsafe { follow_type_id(ntv_ty) };
            if !unsafe { get_type_id::<PrimitiveType>(t).is_null() } {
                self.subtract_primitive(here, ntv_ty);
            } else if !unsafe { get_type_id::<SingletonType>(t).is_null() } {
                self.subtract_singleton(here, unsafe { follow_type_id(ntv_ty) });
            } else if !unsafe { get_type_id::<ExternType>(t).is_null() } {
                let res = self.intersect_normal_with_negation_ty(t, here);
                if should_early_exit(res) {
                    here.tyvars = tyvars;
                    return res;
                }
            } else if !unsafe { get_type_id::<UnionType>(t).is_null() } {
                let options = unsafe { (*get_type_id::<UnionType>(t)).options.clone() };
                for part in options {
                    let res = self.intersect_normal_with_negation_ty(part, here);
                    if should_early_exit(res) {
                        here.tyvars = tyvars;
                        return res;
                    }
                }
            } else if !unsafe { get_type_id::<AnyType>(t).is_null() } {
                // HACK: Refinements sometimes intersect with ~any under the
                // assumption that it is the same as any.
                here.tyvars = tyvars;
                return NormalizationResult::True;
            } else if !unsafe { get_type_id::<NoRefineType>(t).is_null() } {
                // `*no-refine*` means we will never do anything to affect the intersection.
                here.tyvars = tyvars;
                return NormalizationResult::True;
            } else if !unsafe { get_type_id::<NeverType>(t).is_null() } {
                // intersecting with `~never` is equivalent to intersecting with `unknown` (a noop).
                here.tyvars = tyvars;
                return NormalizationResult::True;
            } else if !unsafe { get_type_id::<UnknownType>(t).is_null() } {
                // intersecting with `~unknown` is equivalent to intersecting with `never`.
                self.clear_normal(here);
                here.tyvars = tyvars;
                return NormalizationResult::True;
            } else if !unsafe { get_type_id::<ErrorType>(t).is_null() } {
                // ~error is still an error.
                let errors = here.errors;
                self.clear_normal(here);
                here.errors = if !unsafe { get_type_id::<ErrorType>(errors).is_null() } {
                    errors
                } else {
                    t
                };
            } else if !unsafe { get_type_id::<NegationType>(t).is_null() } {
                let nt_ty = unsafe { (*get_type_id::<NegationType>(t)).ty };
                here.tyvars = tyvars;
                return self.intersect_normal_with_ty(
                    here,
                    nt_ty,
                    seen_table_prop_pairs,
                    seen_set_types,
                );
            } else {
                // TODO negated unions, intersections, table, and function.
                // Report a TypeError for other types.
                LUAU_ASSERT!(false);
            }
        } else if !unsafe { get_type_id::<NeverType>(there).is_null() } {
            here.extern_types.reset_to_never();
        } else if !unsafe { get_type_id::<NoRefineType>(there).is_null() } {
            // `*no-refine*` means we will never do anything to affect the intersection.
            here.tyvars = tyvars;
            return NormalizationResult::True;
        } else {
            LUAU_ASSERT!(false);
        }

        let res = self.intersect_tyvars_with_ty(
            &mut tyvars,
            there,
            seen_table_prop_pairs,
            seen_set_types,
        );
        if res != NormalizationResult::True {
            here.tyvars = tyvars;
            return res;
        }
        here.tyvars = tyvars;

        NormalizationResult::True
    }
}
