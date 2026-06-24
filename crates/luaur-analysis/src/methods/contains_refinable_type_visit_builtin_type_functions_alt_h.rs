use crate::records::contains_refinable_type::ContainsRefinableType;
use crate::records::negation_type::NegationType;
use crate::type_aliases::type_id::TypeId;

impl ContainsRefinableType {
    pub fn visit_type_id_negation_type(&mut self, _ty: TypeId, _negation: &NegationType) -> bool {
        !self.found
    }
}
