use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::seen_table_prop_pairs::SeenTablePropPairs;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl Normalizer {
    pub fn intersect_tables_with_table(
        &mut self,
        heres: &mut TypeIds,
        there: TypeId,
        seen_table_prop_pairs: &mut SeenTablePropPairs,
        seen_set_types: &mut DenseHashSet<TypeId>,
    ) {
        self.consume_fuel();

        let mut tmp = TypeIds::type_ids();
        let heres_clone = heres.clone();
        for here in heres_clone.order {
            if let Some(inter) =
                self.intersection_of_tables(here, there, seen_table_prop_pairs, seen_set_types)
            {
                tmp.insert_type_id(inter);
            }
        }
        heres.retain(&tmp);
        for ty in tmp.order {
            heres.insert_type_id(ty);
        }
    }
}
