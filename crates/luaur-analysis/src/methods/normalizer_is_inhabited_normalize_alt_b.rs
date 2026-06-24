use crate::enums::normalization_result::NormalizationResult;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::normalized_type::NormalizedType;
use crate::records::normalizer::Normalizer;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::FFlag;

impl Normalizer {
    pub fn is_inhabited_normalized_type_set_type_id(
        &mut self,
        norm: &NormalizedType,
        seen: &mut DenseHashSet<TypeId>,
    ) -> NormalizationResult {
        if FFlag::LuauIntegerType2.get() {
            if unsafe { get_type_id::<NeverType>(norm.tops).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.booleans).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.errors).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.nils).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.numbers).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.threads).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.buffers).is_null() }
                || !norm.extern_types.is_never()
                || unsafe { get_type_id::<NeverType>(norm.integers).is_null() }
                || !norm.strings.is_never()
                || !norm.functions.is_never()
            {
                return NormalizationResult::True;
            }
        } else {
            if unsafe { get_type_id::<NeverType>(norm.tops).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.booleans).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.errors).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.nils).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.numbers).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.threads).is_null() }
                || unsafe { get_type_id::<NeverType>(norm.buffers).is_null() }
                || !norm.extern_types.is_never()
                || !norm.strings.is_never()
                || !norm.functions.is_never()
            {
                return NormalizationResult::True;
            }
        }

        for (_, intersect) in &norm.tyvars {
            let res = self.is_inhabited_normalized_type_set_type_id(intersect, seen);
            if res != NormalizationResult::False {
                return res;
            }
        }

        for &table in &norm.tables.order {
            let res = self.is_inhabited_type_id_set_type_id(table, seen);
            if res != NormalizationResult::False {
                return res;
            }
        }

        NormalizationResult::False
    }
}
