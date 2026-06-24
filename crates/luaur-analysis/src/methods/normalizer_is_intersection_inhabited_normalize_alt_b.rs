use crate::enums::normalization_result::NormalizationResult;
use crate::functions::follow_type::follow_type_id;
use crate::records::normalized_extern_type::NormalizedExternType;
use crate::records::normalized_function_type::NormalizedFunctionType;
use crate::records::normalized_string_type::NormalizedStringType;
use crate::records::normalized_type::NormalizedType;
use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::seen_table_prop_pairs::SeenTablePropPairs;
use crate::type_aliases::type_id::TypeId;
use alloc::collections::BTreeMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl Normalizer {
    pub fn is_intersection_inhabited_type_id_type_id_seen_table_prop_pairs_set_type_id(
        &mut self,
        left: TypeId,
        right: TypeId,
        seen_table_prop_pairs: &mut SeenTablePropPairs,
        seen_set: &mut DenseHashSet<TypeId>,
    ) -> NormalizationResult {
        self.consume_fuel();

        let left = unsafe { follow_type_id(left) };
        let right = unsafe { follow_type_id(right) };

        if self.cache_inhabitance {
            if let Some(result) = self.cached_is_inhabited_intersection.find(&(left, right)) {
                return if *result {
                    NormalizationResult::True
                } else {
                    NormalizationResult::False
                };
            }
        }

        let never_type = unsafe { (*self.builtin_types).neverType };
        let mut norm = NormalizedType {
            builtin_types: self.builtin_types,
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

        let res = self.normalize_intersections(
            &alloc::vec::Vec::from([left, right]),
            &mut norm,
            seen_table_prop_pairs,
            seen_set,
        );

        if res != NormalizationResult::True {
            if self.cache_inhabitance && res == NormalizationResult::False {
                *self
                    .cached_is_inhabited_intersection
                    .get_or_insert((left, right)) = false;
            }
            return res;
        }

        let result = self.is_inhabited_normalized_type_set_type_id(&norm, seen_set);

        if self.cache_inhabitance {
            if result == NormalizationResult::True {
                *self
                    .cached_is_inhabited_intersection
                    .get_or_insert((left, right)) = true;
            } else if result == NormalizationResult::False {
                *self
                    .cached_is_inhabited_intersection
                    .get_or_insert((left, right)) = false;
            }
        }

        norm.normalized_type_destructor();

        result
    }
}
