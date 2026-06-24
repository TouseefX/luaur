use crate::enums::normalization_result::NormalizationResult;
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
    pub fn normalize_intersections(
        &mut self,
        intersections: &alloc::vec::Vec<TypeId>,
        out_type: &mut NormalizedType,
        seen_table_prop_pairs: &mut SeenTablePropPairs,
        seen_set: &mut DenseHashSet<TypeId>,
    ) -> NormalizationResult {
        if self.arena.is_null() {
            panic!("Normalizing types outside a module");
        }

        self.consume_fuel();

        let never_type = unsafe { (*self.builtin_types).neverType };
        let mut norm = NormalizedType {
            builtin_types: self.builtin_types,
            tops: unsafe { (*self.builtin_types).unknownType },
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

        for &ty in intersections {
            let res = self.intersect_normal_with_ty(&mut norm, ty, seen_table_prop_pairs, seen_set);
            if res != NormalizationResult::True {
                return res;
            }
        }

        let res = self.union_normals(out_type, &norm, -1);
        if res != NormalizationResult::True {
            return res;
        }

        NormalizationResult::True
    }
}
