use crate::functions::get_singleton_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_top::is_top;
use crate::functions::reset_to_top::reset_to_top;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::never_type::NeverType;
use crate::records::normalized_extern_type::NormalizedExternType;
use crate::records::normalized_function_type::NormalizedFunctionType;
use crate::records::normalized_string_type::NormalizedStringType;
use crate::records::normalized_type::NormalizedType;
use crate::records::normalizer::Normalizer;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::type_ids::TypeIds;
use alloc::collections::BTreeMap;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl Normalizer {
    pub fn negate_normal(&mut self, here: &NormalizedType) -> Option<NormalizedType> {
        self.consume_fuel();

        let never_type = unsafe { (*here.builtin_types).neverType };
        let mut result = NormalizedType {
            builtin_types: here.builtin_types,
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
        };
        result.is_cacheable = here.is_cacheable;

        let here_tops = unsafe { get_type_id::<NeverType>(here.tops) };
        if here_tops.is_null() {
            return Some(result);
        }

        let here_errors = unsafe { get_type_id::<NeverType>(here.errors) };
        if here_errors.is_null() {
            result.errors = here.errors;
            return Some(result);
        }

        let here_booleans = here.booleans;
        let here_booleans_tv = unsafe { get_type_id::<NeverType>(here_booleans) };
        if !here_booleans_tv.is_null() {
            result.booleans = unsafe { (*here.builtin_types).booleanType };
        } else {
            let here_booleans_tv = unsafe { get_type_id::<PrimitiveType>(here_booleans) };
            if !here_booleans_tv.is_null() {
                result.booleans = unsafe { (*here.builtin_types).neverType };
            } else {
                let here_booleans_tv = unsafe { get_type_id::<SingletonType>(here_booleans) };
                if !here_booleans_tv.is_null() {
                    let boolean = get_singleton_type::<BooleanSingleton>(here_booleans_tv);
                    LUAU_ASSERT!(!boolean.is_null());
                    if unsafe { &*boolean }.value {
                        result.booleans = unsafe { (*here.builtin_types).falseType };
                    } else {
                        result.booleans = unsafe { (*here.builtin_types).trueType };
                    }
                }
            }
        }

        let extern_types = &here.extern_types;
        if extern_types.is_never() {
            reset_to_top(unsafe { &*here.builtin_types }, &mut result.extern_types);
        } else if is_top(unsafe { &*here.builtin_types }, extern_types) {
            result.extern_types.reset_to_never();
        } else {
            let mut root_negations = TypeIds::type_ids();

            for (here_parent, here_negations) in &extern_types.extern_types {
                if *here_parent != unsafe { (*here.builtin_types).externType } {
                    root_negations.insert_type_id(*here_parent);
                }

                for &here_negation in &here_negations.order {
                    self.union_extern_types_with_extern_type_normalized_extern_type_type_id(
                        &mut result.extern_types,
                        here_negation,
                    );
                }
            }

            if !root_negations.empty() {
                result
                    .extern_types
                    .push_pair(unsafe { (*here.builtin_types).externType }, root_negations);
            }
        }

        let here_nils = unsafe { get_type_id::<NeverType>(here.nils) };
        if !here_nils.is_null() {
            result.nils = unsafe { (*here.builtin_types).nilType };
        } else {
            result.nils = unsafe { (*here.builtin_types).neverType };
        }

        let here_numbers = unsafe { get_type_id::<NeverType>(here.numbers) };
        if !here_numbers.is_null() {
            result.numbers = unsafe { (*here.builtin_types).numberType };
        } else {
            result.numbers = unsafe { (*here.builtin_types).neverType };
        }

        if FFlag::LuauIntegerType2.get() {
            let here_integers = unsafe { get_type_id::<NeverType>(here.integers) };
            if !here_integers.is_null() {
                result.integers = unsafe { (*here.builtin_types).integerType };
            } else {
                result.integers = unsafe { (*here.builtin_types).neverType };
            }
        }

        result.strings = here.strings.clone();
        result.strings.isCofinite = !result.strings.isCofinite;

        let here_threads = unsafe { get_type_id::<NeverType>(here.threads) };
        if !here_threads.is_null() {
            result.threads = unsafe { (*here.builtin_types).threadType };
        } else {
            result.threads = unsafe { (*here.builtin_types).neverType };
        }

        let here_buffers = unsafe { get_type_id::<NeverType>(here.buffers) };
        if !here_buffers.is_null() {
            result.buffers = unsafe { (*here.builtin_types).bufferType };
        } else {
            result.buffers = unsafe { (*here.builtin_types).neverType };
        }

        if here.functions.is_never() {
            result.functions.reset_to_top();
        } else if here.functions.is_top {
            result.functions.reset_to_never();
        } else {
            return None;
        }

        if here.tables.empty() {
            result
                .tables
                .insert_type_id(unsafe { (*here.builtin_types).tableType });
        } else if here.tables.size() == 1
            && here.tables.front() == unsafe { (*here.builtin_types).tableType }
        {
            result.tables.clear();
        } else {
            return None;
        }

        Some(result)
    }
}
