use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::seen_table_prop_pairs::SeenTablePropPairs;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl Normalizer {
    pub fn intersect_tables(&mut self, heres: &mut TypeIds, theres: &TypeIds) {
        self.consume_fuel();

        let mut tmp = TypeIds::type_ids();
        for &here in &heres.order {
            for &there in &theres.order {
                let mut seen_set_types: DenseHashSet<TypeId> =
                    DenseHashSet::new(core::ptr::null_mut());
                let mut seen_table_prop_pairs =
                    SeenTablePropPairs::new((core::ptr::null(), core::ptr::null()));
                if let Some(inter) = self.intersection_of_tables(
                    here,
                    there,
                    &mut seen_table_prop_pairs,
                    &mut seen_set_types,
                ) {
                    tmp.insert_type_id(inter);
                }
            }
        }

        heres.retain(&tmp);
        for ty in tmp.order {
            heres.insert_type_id(ty);
        }
    }
}
