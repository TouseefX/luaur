//! Source: `Analysis/src/Normalize.cpp:3244-3317` (hand-ported)
use crate::enums::normalization_result::NormalizationResult;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_shallow_inhabited::is_shallow_inhabited;
use crate::functions::tyvar_index::tyvar_index;
use crate::records::never_type::NeverType;
use crate::records::normalized_type::NormalizedType;
use crate::records::normalizer::Normalizer;
use crate::type_aliases::type_id::TypeId;
use alloc::boxed::Box;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
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

impl Normalizer {
    // See above for an explanation of `ignoreSmallerTyvars`.
    pub fn intersect_normals(
        &mut self,
        here: &mut NormalizedType,
        there: &NormalizedType,
        ignore_smaller_tyvars: i32,
    ) -> NormalizationResult {
        let _rc = RcGuard::new(unsafe { &mut (*self.shared_state).counters.recursion_count });
        if !self.within_resource_limits() {
            return NormalizationResult::HitLimits;
        }

        self.consume_fuel();

        if unsafe { get_type_id::<NeverType>(there.tops).is_null() } {
            here.tops = self.intersection_of_tops(here.tops, there.tops);
            return NormalizationResult::True;
        } else if unsafe { get_type_id::<NeverType>(here.tops).is_null() } {
            self.clear_normal(here);
            return self.union_normals(here, there, ignore_smaller_tyvars);
        }

        for (tyvar, _inter) in there.tyvars.iter() {
            let index = tyvar_index(*tyvar);
            if ignore_smaller_tyvars < index {
                let fresh = !here.tyvars.contains_key(tyvar);
                if fresh {
                    let mut entry = Box::new(fresh_normalized_type(self.builtin_types));
                    let res = self.union_normals(&mut entry, here, index);
                    if res != NormalizationResult::True {
                        return res;
                    }
                    here.tyvars.insert(*tyvar, entry);
                }
            }
        }

        here.booleans = self.intersection_of_bools(here.booleans, there.booleans);

        self.intersect_extern_types(&mut here.extern_types, &there.extern_types);
        here.errors = if !unsafe { get_type_id::<NeverType>(there.errors).is_null() } {
            there.errors
        } else {
            here.errors
        };
        here.nils = if !unsafe { get_type_id::<NeverType>(there.nils).is_null() } {
            there.nils
        } else {
            here.nils
        };
        here.numbers = if !unsafe { get_type_id::<NeverType>(there.numbers).is_null() } {
            there.numbers
        } else {
            here.numbers
        };
        if FFlag::LuauIntegerType2.get() {
            here.integers = if !unsafe { get_type_id::<NeverType>(there.integers).is_null() } {
                there.integers
            } else {
                here.integers
            };
        }
        self.intersect_strings(&mut here.strings, &there.strings);
        here.threads = if !unsafe { get_type_id::<NeverType>(there.threads).is_null() } {
            there.threads
        } else {
            here.threads
        };
        here.buffers = if !unsafe { get_type_id::<NeverType>(there.buffers).is_null() } {
            there.buffers
        } else {
            here.buffers
        };
        self.intersect_functions(&mut here.functions, &there.functions);
        self.intersect_tables(&mut here.tables, &there.tables);

        let tyvar_keys: Vec<TypeId> = here.tyvars.keys().copied().collect();
        for tyvar in tyvar_keys {
            let mut inter = match here.tyvars.remove(&tyvar) {
                Some(b) => b,
                None => continue,
            };
            let index = tyvar_index(tyvar);
            LUAU_ASSERT!(ignore_smaller_tyvars < index);
            let res = match there.tyvars.get(&tyvar) {
                None => self.intersect_normals(&mut inter, there, index),
                Some(found) => self.intersect_normals(&mut inter, found, index),
            };
            if res != NormalizationResult::True {
                here.tyvars.insert(tyvar, inter);
                return res;
            }
            if is_shallow_inhabited(&inter) {
                here.tyvars.insert(tyvar, inter);
            }
            // else: drop `inter`, removing the entry (C++ `here.tyvars.erase(it)`).
        }
        NormalizationResult::True
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
        extern_types: crate::records::normalized_extern_type::NormalizedExternType {
            extern_types: alloc::collections::BTreeMap::new(),
            shape_extensions: crate::records::type_ids::TypeIds::type_ids(),
            ordering: Vec::new(),
        },
        errors: never_type,
        nils: never_type,
        numbers: never_type,
        integers: never_type,
        strings: crate::records::normalized_string_type::NormalizedStringType::never,
        threads: never_type,
        buffers: never_type,
        tables: crate::records::type_ids::TypeIds::type_ids(),
        functions: crate::records::normalized_function_type::NormalizedFunctionType {
            is_top: false,
            parts: crate::records::type_ids::TypeIds::type_ids(),
        },
        tyvars: alloc::collections::BTreeMap::new(),
        is_cacheable: true,
    }
}
