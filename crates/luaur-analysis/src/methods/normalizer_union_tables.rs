use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn union_tables(&mut self, heres: &mut TypeIds, theres: &TypeIds) {
        self.consume_fuel();

        for there in theres.order.iter() {
            let there = *there;
            let builtin_types = unsafe { &*self.builtin_types };
            if there == builtin_types.tableType {
                heres.clear();
                heres.insert_type_id(there);
                return;
            } else {
                self.union_tables_with_table(heres, there);
            }
        }
    }
}
