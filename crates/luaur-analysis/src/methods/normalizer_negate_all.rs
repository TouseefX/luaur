use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn negate_all(&mut self, theres: &TypeIds) -> TypeIds {
        self.consume_fuel();

        let mut tys = TypeIds::type_ids();
        for there in theres.order.iter() {
            let there = *there;
            tys.insert_type_id(self.negate(there));
        }
        tys
    }
}
