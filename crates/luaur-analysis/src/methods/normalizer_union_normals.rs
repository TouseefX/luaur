use crate::enums::normalization_result::NormalizationResult;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::tyvar_index::tyvar_index;
use crate::records::never_type::NeverType;
use crate::records::normalized_extern_type::NormalizedExternType;
use crate::records::normalized_function_type::NormalizedFunctionType;
use crate::records::normalized_string_type::NormalizedStringType;
use crate::records::normalized_type::NormalizedType;
use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::error_type::ErrorType;
use alloc::collections::BTreeMap;
use luaur_common::FFlag;

impl Normalizer {
    pub fn union_normals(
        &mut self,
        here: &mut NormalizedType,
        there: &NormalizedType,
        ignore_smaller_tyvars: i32,
    ) -> NormalizationResult {
        self.consume_fuel();

        here.is_cacheable &= there.is_cacheable;

        let mut tops = self.union_of_tops(here.tops, there.tops);
        if !unsafe { get_type_id::<UnknownType>(tops).is_null() }
            && (!unsafe { get_type_id::<ErrorType>(here.errors).is_null() }
                || !unsafe { get_type_id::<ErrorType>(there.errors).is_null() })
        {
            tops = unsafe { (*here.builtin_types).anyType };
        }

        if unsafe { get_type_id::<NeverType>(tops).is_null() } {
            self.clear_normal(here);
            here.tops = tops;
            return NormalizationResult::True;
        }

        for (tyvar, inter_box) in &there.tyvars {
            let index = tyvar_index(*tyvar);
            if index <= ignore_smaller_tyvars {
                continue;
            }

            if !here.tyvars.contains_key(tyvar) {
                let never_type = unsafe { (*here.builtin_types).neverType };
                let mut fresh = NormalizedType {
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
                let res = self.union_normals(&mut fresh, here, index);
                if res != NormalizationResult::True {
                    return res;
                }
                here.tyvars.insert(*tyvar, Box::new(fresh));
            }

            if let Some(mut current) = here.tyvars.remove(tyvar) {
                let res = self.union_normals(&mut current, inter_box, index);
                if res != NormalizationResult::True {
                    return res;
                }
                here.tyvars.insert(*tyvar, current);
            }
        }

        here.booleans = self.union_of_bools(here.booleans, there.booleans);
        self.union_extern_types_normalized_extern_type_normalized_extern_type(
            &mut here.extern_types,
            &there.extern_types,
        );

        here.errors = if !unsafe { get_type_id::<NeverType>(there.errors).is_null() } {
            here.errors
        } else {
            there.errors
        };
        here.nils = if !unsafe { get_type_id::<NeverType>(there.nils).is_null() } {
            here.nils
        } else {
            there.nils
        };
        here.numbers = if !unsafe { get_type_id::<NeverType>(there.numbers).is_null() } {
            here.numbers
        } else {
            there.numbers
        };
        if FFlag::LuauIntegerType2.get() {
            here.integers = if !unsafe { get_type_id::<NeverType>(there.integers).is_null() } {
                here.integers
            } else {
                there.integers
            };
        }
        self.union_strings(&mut here.strings, &there.strings);
        here.threads = if !unsafe { get_type_id::<NeverType>(there.threads).is_null() } {
            here.threads
        } else {
            there.threads
        };
        here.buffers = if !unsafe { get_type_id::<NeverType>(there.buffers).is_null() } {
            here.buffers
        } else {
            there.buffers
        };
        self.union_functions(&mut here.functions, &there.functions);
        self.union_tables(&mut here.tables, &there.tables);

        NormalizationResult::True
    }
}
